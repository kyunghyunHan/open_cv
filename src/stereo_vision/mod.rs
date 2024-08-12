use num_traits::ops::wrapping::WrappingMul;
use opencv::{
    calib3d::StereoSGBM,
    core::{normalize, Mat, NormTypes::NORM_MINMAX, CV_8U,no_array},
    highgui::{create_trackbar, imshow, named_window, wait_key, WINDOW_AUTOSIZE,destroy_all_windows},
    imgcodecs::{imread, ImreadModes::IMREAD_GRAYSCALE},
    imgproc::apply_color_map,
    prelude::{MatTraitConst, StereoMatcherTrait},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;

// const DISPARITY_WINDOW: &str = "disparity";

// fn calculate_difference() -> Result<()> {
//     let img_l = imread("./img/bike0.png", 0)?;
//     let img_r = imread("./img/bike2.png", 0)?;

//     let img_l = Arc::new(Mutex::new(img_l));
//     let img_r = Arc::new(Mutex::new(img_r));

//     named_window(DISPARITY_WINDOW, WINDOW_AUTOSIZE)?;

//     let img_l_clone = Arc::clone(&img_l);
//     let img_r_clone = Arc::clone(&img_r);
//     let num_disparity = Arc::new(Mutex::new(8));
//     let num_disparity_clone = Arc::clone(&num_disparity);

//     let block_size =Arc::new(Mutex::new(5));
//     let block_size_clone =Arc::clone(&block_size);

//     let disp = Mat::default();
//     let disparity = Mat::default();

//     let disp = Arc::new(Mutex::new(disp));
//     let disp_clone = Arc::clone(&disp);

//     let disparity = Arc::new(Mutex::new(disparity));
//     let disparity_clone = Arc::clone(&disparity);
//     let stereo = Arc::new(Mutex::new(StereoSGBM::create_def()?));

//     let stereo_clone = Arc::clone(&stereo);

//     create_trackbar(
//         "numDisparities",
//         DISPARITY_WINDOW,
//         None,
//         18,
//         Some(Box::new(move |num_disparity| {
//             let mut stereo = stereo_clone.lock().unwrap();
//             let block_size= block_size_clone.lock().unwrap();

//             stereo.set_num_disparities(*block_size).unwrap();
//             let mut num_disparity =num_disparity_clone.lock().unwrap();
//             * num_disparity = *num_disparity.wrapping_mul(16);

//             let mut disparity_guard = disparity_clone.lock().unwrap();
//             let img_l: std::sync::MutexGuard<Mat> = img_l_clone.lock().unwrap();
//             let img_r = img_r_clone.lock().unwrap();
//             let mut disp_clone = disp_clone.lock().unwrap();

//             stereo.compute(&*img_l, &*img_r, &mut *disp_clone).unwrap();
//             disp_clone
//                 .convert_to(&mut *disparity_guard, CV_8U, 1., 0.)
//                 .unwrap();
//             let mut disparity_color = Mat::default();
//             apply_color_map(
//                 &*disparity_guard,
//                 &mut disparity_color,
//                 opencv::imgproc::COLORMAP_JET,
//             )
//             .unwrap();
//             // drop(disparity_cloee);
//             imshow(DISPARITY_WINDOW, &disparity_color).unwrap();
//         })),
//     )?;
//     let stereo_clone = Arc::clone(&stereo);
//     let disparity_clone = Arc::clone(&disparity);
//     let img_l_clone = Arc::clone(&img_l);
//     let img_r_clone = Arc::clone(&img_r);
//     let disp_clone = Arc::clone(&disp);
//     let block_size_clone =Arc::clone(&block_size);

//     create_trackbar(
//         "blok_size",
//         DISPARITY_WINDOW,
//         None,
//         18,
//         Some(Box::new(move |blocksize| {
//             let mut stereo = stereo_clone.lock().unwrap();
//             let block_size= block_size_clone.lock().unwrap();
//             stereo.v(*block_size ).unwrap();
//             let blocksize = *block_size;

//             let mut disparity_guard = disparity_clone.lock().unwrap();
//             let img_l: std::sync::MutexGuard<Mat> = img_l_clone.lock().unwrap();
//             let img_r = img_r_clone.lock().unwrap();
//             let mut disp_clone = disp_clone.lock().unwrap();
//             stereo.compute(&*img_l, &*img_r, &mut *disp_clone).unwrap();
//             disp_clone
//                 .convert_to(&mut *disparity_guard, CV_8U, 1., 0.)
//                 .unwrap();
//             let mut disparity_color = Mat::default();
//             apply_color_map(
//                 &*disparity_guard,
//                 &mut disparity_color,
//                 opencv::imgproc::COLORMAP_JET,
//             )
//             .unwrap();
//             imshow(DISPARITY_WINDOW, &disparity_color).unwrap();
//         })),
//     )?;
//     // named_window(DISPARITY_WINDOW,WINDOW_AUTOSIZE )?;

//     wait_key(0)?;
//     Ok(())
// }

fn basics() -> Result<()> {
    let img_l: Mat = imread("./img/body1.png", 0)?;
    let img_r: Mat = imread("./img/body2.png", 0)?;

    let mut stereo = StereoSGBM::create_def()?;
    stereo.set_num_disparities(16)?;
    stereo.set_block_size(15)?;
    let mut disp = Mat::default();
    stereo.compute(&img_l, &img_r, &mut disp).unwrap();

    let mut disp_8u = Mat::default();
    normalize(
        &disp,
        &mut disp_8u,
        0.0,
        255.0,
        NORM_MINMAX as i32,
        CV_8U,
        &no_array(),
    )?;

    imshow("Disparity", &disp_8u).unwrap();
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

pub fn main() -> Result<()> {
    basics()?;
    // calculate_difference()?;
    Ok(())
}
