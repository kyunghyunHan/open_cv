use std::os::raw::c_void;
/*영상의 밝기 조절 */
use opencv::{
    core::{self, Mat, MatTrait, MatTraitConst, Vec3b},
    highgui::{self, create_trackbar},
    imgcodecs::{self, IMREAD_COLOR, IMREAD_GRAYSCALE},
    Result,
};
use std::sync::{Arc, Mutex};
pub fn main() -> Result<()> {
    // brihtness()?;
    // brihtness2()?;
    brihtness4()?;
    Ok(())
}

fn brihtness() -> Result<()> {
    // Load the source image
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_COLOR)?;

    if src.empty() {
        println!("{}", "error");
        return Ok(());
    }

    // Create a new Mat to store the result
    let mut result = Mat::default();

    // Adjust brightness by adding a constant value to each pixel
    let brightness_value = 100;
    src.convert_to(&mut result, -1, 1.0, brightness_value as f64)?;

    // Display the original and adjusted images
    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &result)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn brihtness2() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "image load fiiled");
    }
    let mut dst = src.clone();

    for j in 0..src.rows() {
        for i in 0..src.cols() {
            /*픽셀 255까지 */

            *dst.at_2d_mut::<u8>(j, i)? = *src.at_2d::<u8>(j, i)? + 8;
        }
    }
    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn brihtness4() -> Result<()> {
    let img = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face.jpg",
        IMREAD_COLOR,
    )?));
    let img_clone = Arc::clone(&img);

    let window = "face detection";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    create_trackbar(
        "face detection",
        "face detection",
        None,
        100,
        Some(Box::new({
            move |val| {
                let  img_guard = img_clone.lock().unwrap();
                let brightness_value = val as f64;
                let mut result = core::Mat::default();
                img_guard.convert_to(&mut result, -1, 1.0, brightness_value).unwrap();
                highgui::imshow(window, &result).unwrap();
            }
        })),
    )?;

    highgui::imshow("face detection", &*img.lock().unwrap())?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

