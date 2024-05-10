use opencv::{
    core::{self, no_array, MatExprTraitConst, MatTraitConst},
    highgui, imgproc,
    prelude::{MatTrait, VideoCaptureTrait},
    video, videoio, Result,
};
use rand::Rng;
use std::collections::HashMap;

pub fn main() -> Result<()> {
    klt_algorithm()?;
    Ok(())
}

fn klt_algorithm() -> Result<()> {
    let mut cap =
        videoio::VideoCapture::from_file("./video/slow_traffic_small.mp4", videoio::CAP_ANY)?;
    let mut feature_params: HashMap<&str, i32> = HashMap::new();

    // 키-값 쌍 추가
    feature_params.insert("maxCorners", 100);
    feature_params.insert("qualityLevel", 3);
    feature_params.insert("minDistance", 7);
    feature_params.insert("blockSize", 7);

    let mut lk_params: HashMap<&str, (i32, i32, f64)> = HashMap::new();

    // 키-값 쌍 추가
    lk_params.insert("winSize", (15, 15, 0.));
    lk_params.insert("maxLevel", (2, 0, 0.));
    lk_params.insert("criteria", (0, 10, 0.03));

    let (mut ret, mut old_frame) = (core::Mat::default(), core::Mat::default());

    cap.read(&mut ret)?;
    cap.read(&mut old_frame)?;
    let mut old_gray = core::Mat::default();
    imgproc::cvt_color(&old_frame, &mut old_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut p0 = core::Mat::default();
    imgproc::good_features_to_track(
        &old_gray,
        &mut p0,
        100,
        0.3,
        7.,
        &no_array(),
        7,
        false,
        0.,
    )?;

    let mut mask = core::Mat::zeros_size(old_frame.size()?, core::CV_8UC3)?.to_mat()?;

    loop {
        let mut frame = core::Mat::default();
        cap.read(&mut ret)?;
        cap.read(&mut frame)?;

        if ret.empty() {
            break;
        }

        let mut new_gray = core::Mat::default();
        imgproc::cvt_color(&frame, &mut new_gray, imgproc::COLOR_BGR2GRAY, 0)?;

        let mut p1 = core::Mat::default();
        let mut status = core::Mat::default();
        let mut err = core::Mat::default();
        println!("{:?}",old_gray);
        println!("{:?}",new_gray);
        video::calc_optical_flow_pyr_lk(
            &old_gray,
            &new_gray,
            &p0,
            &mut p1,
            &mut status,
            &mut err,
            core::Size_::from((15, 15)),
            2,
            core::TermCriteria::from(core::TermCriteria {
                typ: core::TermCriteria_COUNT | core::TermCriteria_EPS,
                max_count: 10,
                epsilon: 0.03,
            }),
            core::CV_32F,
            1.,
        )?;
        println!("{}",11);

        let mut good_new = core::Mat::default();
        let mut good_old = core::Mat::default();

        for i in 0..p1.rows() {
            let value = *status.at_2d::<u8>(i, 0)?;
            if value == 1 {
                let row = p1.row(i)?;
                good_new.push_back(&row)?;
                let row_old = p0.row(i)?;
                good_old.push_back(&row_old)?;
            }
        }
  
        for i in 0..good_new.size()?.width {
            let (a, b) = (good_new.at_2d::<i32>(i, 0)?, good_new.at_2d::<i32>(i, 1)?);
            let (c, d) = (good_old.at_2d::<i32>(i, 0)?, good_old.at_2d::<i32>(i, 1)?);
            imgproc::line(
                &mut mask,
                core::Point_::from((*a, *b)),
                core::Point_::from((*c, *d)),
                core::Scalar::from((0, 0, 255)),
                2,
                imgproc::LINE_AA,
                0,
            )?;
            imgproc::circle(
                &mut frame,
                core::Point_::from((*a, *b)),
                5,
                core::Scalar::from((0, 0, 255)),
                -1,
                imgproc::LINE_AA,
                0,
            )?;
        }
  
        let mut img = core::Mat::default();
        core::add(&frame, &mask, &mut img, &no_array(), 1)?;
        highgui::imshow("LTK tracker", &img)?;
        highgui::wait_key(30)?;
        new_gray.copy_to(&mut old_gray)?;
        println!("{}",11);


        good_new.copy_to(&mut p0)?;
        println!("{}",11);

    }

    highgui::destroy_all_windows()?;
    Ok(())
}

fn farn_back() -> Result<()> {
    Ok(())
}
