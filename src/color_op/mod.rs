use opencv::{
    core::{self, MatTrait, MatTraitConst},
    highgui, imgcodecs, imgproc, Result,
};
use std::{
    env::home_dir,
    sync::{Arc, Mutex},
};

/*
주요 색 공간 코드
COLOR_BGR2RGB or COLOR_RGB2BGR BGR채널순서와 RGB채널 순서를 상호 변경
COLOR_BGR2GRAY  =  3채널 BGR컬러를 Gray로
COLOR_GRAY2BGR  =  1채널 그레이스케일 영상을3채널 BGR로
COLOR_BGR2XYX  =  BGR색공간을 CIE XYZ색공간으로
COLOR_XYZ2BGR  =  CIE XYZ 색 공간을 BGR색 공간으로
COLOR_BGR2YCrCb  =  BFR 색 공간을 YCrCb색 공간으로
COLOR_YCrCb2BGR  =  YCrCb색 공간을 BGR생 공간으로
COLOR_BGR2HSV  =  BGR색 공간을 HSV색 공간으로
COLOR_HSV2BGR  =  HSV색 공간을 BGR로
COLOR_BGR2Lab  =  BGR색 공간을 CIE Lab공간으로
COLOR_Lab2BGR  =   CIE Lab공간을 BGR색 공간으로
컬러 영상을 그레이로 변경->연산속도와 메모리 사용량 줄이기 위함
HSV=>색상,채도,명도로 색을 표현
YCrCb=>휘도,색차
*/
pub fn main() -> Result<()> {
    inrange()?;
    Ok(())
}
fn color_inverse() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }

    let mut dst = src.clone();
    // // let mut dst = core::Mat::default();
    // imgproc::cvt_color(&src, &mut dst, imgproc::COLOR_BGR2RGB,0)?;
    for j in 0..src.rows() {
        for i in 0..src.cols() {
            let p1 = src.at_2d::<core::Vec3b>(j, i)?;
            let p2 = dst.at_2d_mut::<core::Vec3b>(j, i)?;

            *p2.get_mut(0).unwrap() = 255 - *p1.get(0).unwrap(); //b
            *p2.get_mut(1).unwrap() = 255 - *p1.get(1).unwrap(); //g
            *p2.get_mut(2).unwrap() = 255 - *p1.get(2).unwrap(); //r
        }
    }

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

/*컬러영상을 그레이 스케일로변경 */

fn color_grayscale() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }

    let mut dst = src.clone();
    imgproc::cvt_color(&src, &mut dst, imgproc::COLOR_BGR2GRAY, 0)?;
    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

/*BGR컬러 영상의 채널 나누기 */
fn color_split() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }

    let mut bgr_plans: core::Vector<core::Mat> = core::Vector::new();
    core::split(&src, &mut bgr_plans)?;

    highgui::imshow("src", &src)?;
    highgui::imshow("B_Plane", &bgr_plans.get(0)?)?;
    highgui::imshow("G_Plane", &bgr_plans.get(1)?)?;
    highgui::imshow("R_Plane", &bgr_plans.get(2)?)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn coloreq() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }
    let mut src_ycrb = core::Mat::default();
    imgproc::cvt_color(&src, &mut src_ycrb, imgproc::COLOR_BGR2YCrCb, 0)?;

    let mut ycrb_planes: core::Vector<core::Mat> = core::Vector::new();
    core::split(&src, &mut ycrb_planes)?;

    imgproc::equalize_hist(&ycrb_planes.get(0)?, &mut ycrb_planes.get(0)?)?;

    let mut dst_ycrb: opencv::prelude::Mat = core::Mat::default();

    core::merge(&ycrb_planes, &mut dst_ycrb)?;

    let mut dst: opencv::prelude::Mat = core::Mat::default();
    imgproc::cvt_color(&dst_ycrb, &mut dst, imgproc::COLOR_YCrCb2BGR, 0)?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn inrange() -> Result<()> {
    let mut upper_hue = 80;
    let mut lower_hue = 40;
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }
    let src_hsv = Arc::new(Mutex::new(core::Mat::default()));
    let src_hsv_clone = Arc::clone(&src_hsv);
    imgproc::cvt_color(
        &src,
        &mut *src_hsv_clone.lock().unwrap(),
        imgproc::COLOR_BGR2HSV,
        0,
    )?;
    highgui::imshow("src", &src)?;

    highgui::named_window("mask", highgui::WINDOW_AUTOSIZE)?;

    highgui::create_trackbar(
        "Lower Hue",
        "mask",
        Some(&mut lower_hue),
        179,
        Some(Box::new(move |x| {
            let x = x;
            let lowerb = core::Scalar::new(x as f64, 100., 0., 0.);
            let upperb = core::Scalar::new(x as f64, 255., 255., 0.);
            let mut mask = core::Mat::default();
            let src_hsv_lock = src_hsv_clone.lock().unwrap();

            core::in_range(&*src_hsv_lock, &lowerb, &upperb, &mut mask).unwrap();
            highgui::imshow("src", &mask).unwrap();
        })),
    )?;
    highgui::create_trackbar(
        "Upper Hue",
        "mask",
        Some(&mut upper_hue),
        179,
        Some(Box::new(move |x| {
            let x = x;
            let lowerb = core::Scalar::new(x as f64, 100., 0., 0.);
            let upperb = core::Scalar::new(x as f64, 255., 255., 0.);
            let mut mask = core::Mat::default();
            let src_hsv_lock = src_hsv.lock().unwrap();

            core::in_range(&*src_hsv_lock, &lowerb, &upperb, &mut mask).unwrap();
            highgui::imshow("src", &mask).unwrap();
        })),
    )?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn backpro() -> Result<()> {
    let ref_=  imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    if ref_.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }
   
    Ok(())
}
