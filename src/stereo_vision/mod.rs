use opencv::{
    calib3d::StereoSGBM,
    core::{Mat, CV_8U},
    highgui::{create_trackbar, imshow, wait_key, named_window, WINDOW_AUTOSIZE},
    imgcodecs::{imread, ImreadModes::IMREAD_GRAYSCALE},
    imgproc::apply_color_map,
    prelude::{MatTraitConst, StereoMatcherTrait},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;

const DISPARITY_WINDOW: &str = "disparity";

fn calculate_difference() -> Result<()> {
    let img_l = imread("./img/bike0.png", 0)?;
    let img_r = imread("./img/im1.png", 0)?;

    let img_l = Arc::new(Mutex::new(img_l));
    let img_r = Arc::new(Mutex::new(img_r));

    named_window(DISPARITY_WINDOW, WINDOW_AUTOSIZE)?;

    let img_l_clone = Arc::clone(&img_l);
    let img_r_clone = Arc::clone(&img_r);
    let mut num_disparity = 8;

    create_trackbar(
        "numDisparities",
        DISPARITY_WINDOW,
        None,
        18,
        Some(Box::new(move |pos| {
            trackbar1(pos, Arc::clone(&img_l_clone), Arc::clone(&img_r_clone)).unwrap();
        })),
    )?;

    wait_key(0)?;
    Ok(())
}

fn trackbar1(num_disparity: i32, img_l: Arc<Mutex<Mat>>, img_r: Arc<Mutex<Mat>>) -> Result<()> {

    let mut stereo = StereoSGBM::create_def()?;
    stereo.set_num_disparities(num_disparity * 16)?;
    let mut disp = Mat::default();
    let mut disparity = Mat::default();

    let img_l = img_l.lock().unwrap();
    let img_r = img_r.lock().unwrap();
    stereo.compute(&*img_l, &*img_r, &mut disp)?;
    disp.convert_to(&mut disparity, CV_8U, 0., 0.)?;
    let mut disparity_color = Mat::default();
    apply_color_map(&disparity, &mut disparity_color, opencv::imgproc::COLORMAP_JET)?;

    imshow(DISPARITY_WINDOW, &disparity_color)?;
    Ok(())
}

pub fn main() -> Result<()> {
    calculate_difference()?;
    Ok(())
}
