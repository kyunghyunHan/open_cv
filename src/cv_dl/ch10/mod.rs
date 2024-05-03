use opencv::{
    core::{self, no_array, MatExprTraitConst, MatTraitConst},
    dnn, highgui, imgcodecs, imgproc,
    prelude::MatTrait,
    prelude::VideoCaptureTrait,
    quality, video, videoio, Result,
};
use rand::Rng;
use std::collections::HashMap;

pub fn main() -> Result<()> {
    klt_algorithm()?;
    Ok(())
}
/*추적 */
fn klt_algorithm() -> Result<()> {
    let mut cap =
        videoio::VideoCapture::from_file("./video/slow_traffic_small.mp4", videoio::CAP_FFMPEG)?;
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

    println!("{:?}", p0);
    imgproc::good_features_to_track(
        &old_gray,
        &mut p0,
        *feature_params.get("maxCorners").unwrap(),
        0.3,
        *feature_params.get("minDistance").unwrap() as f64,
        &no_array(),
        *feature_params.get("blockSize").unwrap(),
        true,
        0.,
    )?;
    println!("{:?}", p0);

    let color: Vec<[u8; 3]> = (0..100)
        .map(|_| {
            [
                rand::thread_rng().gen_range(0..=255),
                rand::thread_rng().gen_range(0..=255),
                rand::thread_rng().gen_range(0..=255),
            ]
        })
        .collect();
    let mask: opencv::prelude::Mat = core::Mat::zeros_size(old_frame.size()?, 0)?.to_mat()?;

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
            1,
            1.,
        )?;
        let mut good_new = core::Mat::default();
        let mut good_old = core::Mat::default();
        good_new.typ();
        let mut good_new_converted = core::Mat::default();
        let mut good_old_converted = core::Mat::default();
        
        // good_new.convert_to(&mut good_new_converted, core::CV_32FC2)?;
        // good_old.convert_to(&mut good_old_converted, core::CV_32FC2)?;
        
        println!("{:?}",p1);
        if !p1.empty() {
            println!("{}",1);
            // p1과 match를 이용하여 조건에 맞는 요소 선택
            for i in 0..p1.rows() {
                let value = p1.at_2d::<f32>(i, 0)?;
                println!("{}",value);
                if value == &1. {
                    let row = p1.row(i).unwrap();
                    println!("{:?}", row);
                    good_new.push_back(&row).unwrap();
                }
            }
            for i in 0..p0.rows() {
                let value = p0.at_2d::<f32>(i, 0)?;
                if value == &1. {
                    let row = p1.row(i).unwrap();
                    good_old.push_back(&row).unwrap();
                }
            }
        }

        highgui::imshow("LTK tracker", &frame)?;
        highgui::wait_key(0)?;
    }
    highgui::destroy_all_windows()?;
    Ok(())
}
