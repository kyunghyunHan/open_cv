use opencv::{
    core::{self, MatExprTraitConst, MatTraitConst}, dnn, highgui, imgcodecs, imgproc, prelude::VideoCaptureTrait, quality,video,  videoio, Result
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
        videoio::VideoCapture::from_file("./video/slow_traffic_small.mp4", videoio::CAP_ANY)?;
    let mut feature_params: HashMap<&str, i32> = HashMap::new();

    // 키-값 쌍 추가
    feature_params.insert("maxCorners", 100);
    feature_params.insert("qualityLevel", 3);
    feature_params.insert("minDistance", 7);
    feature_params.insert("blockSize", 7);

    let mut lk_params: HashMap<&str, (i32, i32, f64)> = HashMap::new();

    // 키-값 쌍 추가
    lk_params.insert("winSize", (15, 15,0.));
    lk_params.insert("maxLevel", (2, 0,0.));
    lk_params.insert("criteria", (0, 10, 0.03));

    let (mut ret, mut old_frame) = (core::Mat::default(), core::Mat::default());
    
    cap.read(&mut ret)?;
    cap.read(&mut old_frame)?;
    
    let color: Vec<[u8; 3]> = (0..100).map(|_| [rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)]).collect();
    let mask = core::Mat::zeros_size(old_frame.size()?, 0)?.to_mat()?;

    loop {
        let mut frame = core::Mat::default();
        let (mut ret, mut frame) = (core::Mat::default(), core::Mat::default());
        if ret.empty(){
            break;
        }
        let mut new_gray =core::Mat::default();
        imgproc::cvt_color(&frame, &mut new_gray, imgproc::COLOR_BGR2GRAY, 0)?;


          
        // video::calc_optical_flow_pyr_lk(prev_img, &new_gray, prev_pts, next_pts, status, err, win_size, max_level, criteria, flags, min_eig_threshold)?:
    }
    highgui::destroy_all_windows()?;
    Ok(())
}
