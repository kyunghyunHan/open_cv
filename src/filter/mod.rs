use opencv::core::{no_array, randn, Size_, CV_32SC1};
use opencv::gapi::{self, gaussian_blur};
use opencv::highgui::wait_key;
use opencv::imgproc::{FONT_HERSHEY_SIMPLEX, LINE_AA};
use opencv::{
    core::{self, Mat, MatTraitConst, Point, Scalar, BORDER_DEFAULT, CV_64F, CV_8UC1},
    highgui,
    imgcodecs::{self, IMREAD_GRAYSCALE},
    imgproc, Result,
};

use opencv::prelude::MatTrait;
use rand::Rng;
use std::process::exit;
pub fn main() -> Result<()> {
    median_filter()?;
    Ok(())
}

fn embossing_filter() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    let mut dst = Mat::default();
    let emboss = Mat::from_slice_2d(&vec![vec![-1, -1, 0], vec![-1, 0, 1], vec![0, 1, 1]]).unwrap();

    if src.empty() {
        exit(0);
    }

    imgproc::filter_2d(
        &src,
        &mut dst,
        -1,
        &emboss,
        Point::new(-1, -1),
        128.0,
        BORDER_DEFAULT,
    )?;
    highgui::imshow("aa1", &src)?;

    highgui::imshow("aa2", &dst)?;
    wait_key(0)?;
    Ok(())
}
fn unsharp_mask() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    highgui::imshow("src", &src)?;
    for sigma in 1..=5 {
        let mut blurrd = Mat::default();
        imgproc::gaussian_blur(
            &src,
            &mut blurrd,
            core::Size::new(0, 0),
            sigma as f64,
            sigma as f64,
            core::BORDER_DEFAULT,
        )?;

        let mut mult_dst1 = core::Mat::default();
        core::multiply(&src, &core::Scalar::all(2.), &mut mult_dst1, 1.0, -1)?;

        let mut mult_dst2 = core::Mat::default();
        core::multiply(&blurrd, &core::Scalar::all(1.), &mut mult_dst2, 1.0, -1)?;

        let mut dst = core::Mat::default();

        core::subtract(&mult_dst1, &mult_dst2, &mut dst, &core::no_array(), -1)?;
        let desc = format!("sigma:{}", sigma);
        imgproc::put_text(
            &mut dst,
            &desc,
            core::Point_::from((10, 30)),
            imgproc::FONT_HERSHEY_SIMPLEX,
            1.,
            Scalar::all(255.),
            1,
            imgproc::LINE_AA,
            false,
        )?;

        highgui::imshow("dst", &dst)?;
        highgui::wait_key(0)?;

        // core::add_weighted(&src, alpha, &core::Mat::default(), beta, 0.0, &mut dst,false)?;
    }

    Ok(())
}
fn mean_filter() -> Result<()> {
    // 그레이스케일 이미지를 읽습니다.
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    // 블러처리된 이미지를 루프 외부에서 순차적으로 표시합니다.
    for i in (3..=7).step_by(2) {
        println!("{}", i);
        let mut dst = Mat::default();
        // 이미지를 블러처리합니다.
        imgproc::blur(
            &src,
            &mut dst,
            core::Size_::new(i, i),
            core::Point::new(-1, -1),
            BORDER_DEFAULT,
        )?;
        // 이미지에 해당하는 커널 크기에 대한 설명을 추가합니다.
        let desc = format!("Mean: {} {}", i, i);
        imgproc::put_text(
            &mut dst,
            &desc,
            core::Point_::from((10, 30)),
            imgproc::FONT_HERSHEY_SIMPLEX,
            1.0,
            Scalar::from((0, 255, 255)),
            1,
            imgproc::LINE_AA,
            false,
        )?;
        // 블러처리된 이미지를 표시합니다.
        highgui::imshow("dst", &dst)?;
        highgui::wait_key(0)?;
    }

    Ok(())
}
fn gaussin_filter() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    let mut dst = Mat::default();
    for simga in 1..=5 {
        imgproc::gaussian_blur(
            &src,
            &mut dst,
            core::Size::new(0, 0),
            simga as f64,
            simga as f64,
            BORDER_DEFAULT,
        )?;
        let desc = format!("Sigma: {}", simga);
        imgproc::put_text(
            &mut dst,
            &desc,
            core::Point_::from((10, 30)),
            imgproc::FONT_HERSHEY_SIMPLEX,
            1.0,
            Scalar::from((0, 255, 255)),
            1,
            imgproc::LINE_AA,
            false,
        )?;
        // 블러처리된 이미지를 표시합니다.
        highgui::imshow("dst", &dst)?;
        wait_key(0)?;
    }
    Ok(())
}

fn noise_gaussian() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    if src.empty() {
        println!("{}", "error");
        std::process::exit(0);
    }
    highgui::imshow("src", &src)?;
    for stddev in (10..=30).step_by(10) {
        let mut noise = core::Mat::new_size_with_default(src.size()?, CV_32SC1, Scalar::from(0))?;
        let stddev = stddev as f64;
        randn(&mut noise, &0., &stddev)?;

        let mut dst = core::Mat::default();
        core::add(&src, &noise, &mut dst, &no_array(), core::CV_8U)?;

        let desc = format!("stddev{}", stddev);
        imgproc::put_text(
            &mut dst,
            &desc,
            Point::from((10, 30)),
            FONT_HERSHEY_SIMPLEX,
            1.,
            Scalar::from(0),
            1,
            LINE_AA,
            false,
        )?;
        highgui::imshow("dst", &dst)?;
        highgui::wait_key(0)?;
    }
    Ok(())
}

fn filter_bilateral() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "image load falied");
        std::process::exit(0);
    }

    let mut noise = core::Mat::new_size_with_default(src.size()?, CV_32SC1, Scalar::from(0))?;
    randn(&mut noise, &0., &5.)?;
    let mut add_src = core::Mat::default();
    core::add(&src, &noise, &mut add_src, &no_array(), core::CV_8U)?;

    let mut dst1 = core::Mat::default();

    imgproc::gaussian_blur(&src, &mut dst1, Size_::default(), 5., 5., 0)?;

    let mut dst2 = core::Mat::default();
    imgproc::bilateral_filter(&src, &mut dst2, -1, 10., 5., 0)?;

    highgui::imshow("dst1", &dst1)?;
    highgui::imshow("dst2", &dst2)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;

    Ok(())
}

fn median_filter() -> Result<()> {
    let mut rng = rand::thread_rng();

    let  mut src:Mat = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }
    let num = src.total() as f64 * 0.1;
    for i in 0..num as usize {
        let x: i32 = rng.gen_range(0..src.cols());
        let y: i32 = rng.gen_range(0..src.rows());
        *src.at_2d_mut::<u8>(y, x)? = (i as u8 %2)*255 as u8;
  
    }
    let mut dst1= Mat::default();
    imgproc::gaussian_blur(&src, &mut dst1, Size_::default(), 1., 0., 0)?;
    let mut dst2 = core::Mat::default();
    imgproc::median_blur(&src, &mut dst2, 3)?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst1", &dst1)?;
    highgui::imshow("dst2", &dst2)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;


    Ok(())
}
