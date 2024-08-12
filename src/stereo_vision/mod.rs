use opencv::{
    calib3d::StereoSGBM,
    core::{no_array, Mat, MatTraitConst, Scalar, CV_8U, CV_8UC1},
    highgui::{create_trackbar, imshow, wait_key},
    imgcodecs::{imread, ImreadModes::IMREAD_GRAYSCALE},
    imgproc::{self, apply_color_map, COLORMAP_JET},
    prelude::{MatExprTraitConst, MatTrait, StereoMatcherTrait},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;
/*깊이추정
disparity =x−x′=Bf/Z



*/
//차이 계산
const disparity_window: &str = "disparity";
const block_size: i32 = 5;
fn calculate_difference() -> Result<()> {
    let mut num_disparity = 8;
    let dsip = Mat::default();
    let disparity = Mat::default();

    let img: Arc<Mutex<Mat>> = Arc::new(Mutex::new(imread("./img/face1.jpeg", 0)?));

    let img_clone = Arc::clone(&img);
    create_trackbar(
        "numDisparities",
        &disparity_window,
        Some(&mut num_disparity),
        18,
        Some(Box::new(move |num_disparity| {
            tarkbar1(num_disparity, Arc::clone(&img_clone)).unwrap();
        })),
    )?;
    wait_key(0)?;
    Ok(())
}
fn tarkbar1(num_disparity: i32, img_clone: Arc<Mutex<Mat>>) -> Result<()> {
    // StereoSGBM::set_num_disparities(&mut self, num_disparities)
    let mut stereo = StereoSGBM::create_def()?;
    stereo.set_num_disparities(num_disparity * 16)?;
    let mut dsip = Mat::default();
    let mut disparity = Mat::default();

    let img_l = Mat::default();
    let img_r = Mat::default();
    stereo.compute(&img_l, &img_r, &mut dsip)?;
    dsip.convert_to(&mut disparity, CV_8U, 0., 0.)?;
    let mut disparity2 = Mat::default();
    apply_color_map(&disparity, &mut disparity2, COLORMAP_JET)?;
    // let mut img_guard = img_clone.lock().unwrap();
    // img_guard.set_to(&Scalar::from(pos * 16), &no_array())?;
    imshow(&disparity_window, &disparity2)?;

    Ok(())
}
pub fn main() -> Result<()> {
    calculate_difference()?;
    Ok(())
}
