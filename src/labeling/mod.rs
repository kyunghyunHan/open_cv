use opencv::{
    core::{self, no_array, Mat, MatTraitConst, Point, Rect_, Scalar, Vector, CV_16S, CV_32S, CV_PI},
    highgui::{self, imshow, wait_key},
    imgcodecs::{self, IMREAD_GRAYSCALE},
    imgproc::{
        self, arc_length, bounding_rect, connected_components_with_stats, connected_components_with_stats_def, contour_area, cvt_color, rectangle, threshold, COLOR_GRAY2BGR, FONT_HERSHEY_PLAIN, THRESH_BINARY, THRESH_BINARY_INV, THRESH_OTSU
    },
    Result,
};
use rand::prelude::*;

use crate::image::put_text;

pub fn main() -> Result<()> {
    polygon()?;
    Ok(())
}
fn labeling_stats() -> Result<()> {
    let mut src = imgcodecs::imread("./keyboard.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("Unable to open default capera!");
    }

    let mut bin = Mat::default();
    threshold(&mut src, &mut bin, 0.0, 255.0, THRESH_BINARY | THRESH_OTSU)?;

    let mut labels = Mat::default();
    let mut stats = Mat::default();
    let mut centroids = Mat::default();

    let cnt = connected_components_with_stats(
        &mut bin,
        &mut labels,
        &mut stats,
        &mut centroids,
        8,
        CV_32S,
    )?;

    let mut dst = Mat::default();
    cvt_color(&src, &mut dst, COLOR_GRAY2BGR, 0)?;

    for i in 1..cnt {
        let p = stats.ptr(i)? as *const i32;
        let x = unsafe { *p.offset(0) as i32 };
        let y = unsafe { *p.offset(1) as i32 };
        let width = unsafe { *p.offset(2) as i32 };
        let height = unsafe { *p.offset(3) as i32 };
        let value = unsafe { *p.offset(4) as i32 };

        // dst 이미지에 사각형을 그립니다.
        let rect = Rect_::new(x, y, width, height);
        println!("{:?}", rect);
        if value < 20 {
            continue;
        }
        rectangle(
            &mut dst,
            Rect_::new(x, y, width, height),
            Scalar::from((0.0, 0.0, 255.0)),
            2,
            imgproc::LINE_AA,
            0,
        )?;
    }
    imshow("src", &src)?;
    imshow("dst", &dst)?;
    let key = wait_key(0)?;
    if key == 27 {
        panic!();
    }
    Ok(())
}
//외각선 검출
fn coutours_basic() -> Result<()> {
    let mut src = imgcodecs::imread("./img/board.png", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("Unable to open default capera!");
    }
    let mut contours: core::Vector<core::Vector<core::Point>> = Vector::new();
    imgproc::find_contours(
        &src,
        &mut contours,
        imgproc::RETR_LIST,
        imgproc::CHAIN_APPROX_NONE,
        core::Point_::default(),
    )?;

    let mut dst = core::Mat::default();
    imgproc::cvt_color(&src, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;
    let mut rng = rand::thread_rng();

    let random_number: f64 = rng.gen_range(0.0..=255.0);

    for i in 0..contours.len() {
        let c = core::Scalar::from((random_number, random_number, random_number, 0.));
        imgproc::draw_contours(
            &mut dst,
            &contours,
            i as i32,
            c,
            2,
            imgproc::LINE_AA,
            &0.,
            0,
            core::Point_::default(),
        )?;
    }
    imshow("src", &src)?;
    imshow("dst", &dst)?;
    let key = wait_key(0)?;
    if key == 27 {
        panic!();
    }
    highgui::destroy_all_windows()?;
    Ok(())
}
fn coutours_hier() -> Result<()> {
    let mut src = imgcodecs::imread("./img/board.png", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("Unable to open default capera!");
    }

    let mut contours: core::Vector<core::Vector<core::Point>> = Vector::new();
    let mut hierachy: Vector<core::Vec4i> = Vector::new();
    imgproc::find_contours_with_hierarchy(
        &src,
        &mut contours,
        &mut hierachy,
        imgproc::RETR_LIST,
        imgproc::CHAIN_APPROX_NONE,
        core::Point_::default(),
    )?;
    let mut dst = core::Mat::default();
    imgproc::cvt_color(&src, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;
    let mut rng = rand::thread_rng();

    let random_number: f64 = rng.gen_range(0.0..=255.0);

    for i in 0..contours.len() {
        let c = core::Scalar::from((random_number, random_number, random_number, random_number));
        imgproc::draw_contours(
            &mut dst,
            &contours,
            i as i32,
            c,
            -1,
            imgproc::LINE_AA,
            &0.,
            0,
            core::Point_::default(),
        )?;
    }
    imshow("src", &src)?;
    imshow("dst", &dst)?;
    let key = wait_key(0)?;
    if key == 27 {
        panic!();
    }
    highgui::destroy_all_windows()?;
    Ok(())
}
fn polygon() -> Result<()> {
    let mut src = imgcodecs::imread("./img/polygon.bmp", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        panic!("Unable to open default capera!");
    }
    let mut gray = core::Mat::default();
    imgproc::cvt_color(&src, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut bin = core::Mat::default();

    imgproc::threshold(&gray, &mut bin, 200., 255., THRESH_BINARY_INV | THRESH_OTSU)?;
    println!("{}",1);
    let mut contours: Vector<Vector<Point>> = Vector::new();
    imgproc::find_contours(
        &bin,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_NONE,
        core::Point_::default(),
    )?;
  

    for mut pts in contours {
        if contour_area(&pts, false)? < 400. {
            continue;
        }

        let mut approx: Vector<Point> = Vector::new();
        imgproc::approx_poly_dp(&pts, &mut approx, arc_length(&pts, true)? * 0.02, true)?;

        let vtc = approx.len();
        println!("{}",vtc);
        if vtc == 3 {
            let rc = bounding_rect(&pts)?;
            rectangle(&mut  src, rc, Scalar::from((0,0,255)), 1, imgproc::LINE_AA, 0)?;
            imgproc::put_text(&mut  src, "TRI", rc.tl(), FONT_HERSHEY_PLAIN, 1., Scalar::from((0,0,255)), 2, imgproc::LINE_AA, false)?;
        }else if vtc==4{
            let rc = bounding_rect(&pts)?;
            rectangle(&mut  src, rc, Scalar::from((0,0,255)), 1, imgproc::LINE_AA, 0)?;
            imgproc::put_text(&mut  src, "RECT", rc.tl(), FONT_HERSHEY_PLAIN, 1., Scalar::from((0,0,255)), 2, imgproc::LINE_AA, false)?;
        }else if vtc>4{
            println!("{}",vtc);
             let len = arc_length(&pts, true)?;
             let area= contour_area(&pts, true)?;
             let ratio = 4.*CV_PI*area/(len*len);
             println!("ratio{}",ratio);
             if ratio >0.8{
                println!("ratio{}",ratio);
                let rc = bounding_rect(&pts)?;
                rectangle(&mut  src, rc, Scalar::from((0,0,255)), 1, imgproc::LINE_AA, 0)?;
                imgproc::put_text(&mut  src, "CIR", rc.tl(), FONT_HERSHEY_PLAIN, 1., Scalar::from((0,0,255)), 2, imgproc::LINE_AA, false)?;
             }
        }
    }
    imshow("src", &src)?;
    let key = wait_key(0)?;
    if key == 27 {
        panic!();
    }
    highgui::destroy_all_windows()?;
    Ok(())
}
