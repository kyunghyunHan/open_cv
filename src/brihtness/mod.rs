/*영상의 밝기 조절

*/
use opencv::{
    core::{
        self, add, add_mat_scalar, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Scalar, Vec3b,
    },
    highgui::{self, create_trackbar, destroy_all_windows, imshow, named_window, wait_key},
    imgcodecs::{self, IMREAD_COLOR, IMREAD_GRAYSCALE},
    Result,
};
use std::sync::{Arc, Mutex};
pub fn main() -> Result<()> {
    // brihtness()?;
    brihtness2()?;
    // brihtness3()?;

    Ok(())
}

fn brihtness() -> Result<()> {
    // Load the source image
    let src = imgcodecs::imread("./img/bike0.png", IMREAD_COLOR)?;

    if src.empty() {
        panic!("{}", "error");
    }

    let dst = add_mat_scalar(&src, Scalar::all(100.))?.to_mat()?;

    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn brihtness2() -> Result<()> {
    let src = imgcodecs::imread("./img/lenna.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("{}", "image load fiiled");
    }
    let mut dst = src.clone();
    // for j in 0..src.rows() {
    //     for i in 0..src.cols() {
    //         let v = *src.at_2d::<u8>(j, i)? as i32 + 100;
    //         *dst.at_2d_mut::<u8>(j, i)? = *src.at_2d::<u8>(j, i)?  + 100;
    //     }
    // }
    for j in 0..src.rows() {
        for i in 0..src.cols() {
            let v = *src.at_2d::<u8>(j, i)? as i32 + 100;
            *dst.at_2d_mut::<u8>(j, i)? = if v > 255 {
                255
            } else if v < 0 {
                0
            } else {
                v as u8
            }
        }
    }

    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn brihtness3() -> Result<()> {
    let img = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/bike0.png",
        IMREAD_COLOR,
    )?));
    let img_clone = Arc::clone(&img);

    let window = "face detection";
    named_window(window, highgui::WINDOW_AUTOSIZE)?;
    create_trackbar(
        "face detection",
        "face detection",
        None,
        100,
        Some(Box::new({
            move |val| {
                let img_guard = img_clone.lock().unwrap();
                let brightness_value = val as f64;
                let mut result = Mat::default();
                img_guard
                    .convert_to(&mut result, -1, 1.0, brightness_value)
                    .unwrap();
                imshow(window, &result).unwrap();
            }
        })),
    )?;

    imshow("face detection", &*img.lock().unwrap())?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
