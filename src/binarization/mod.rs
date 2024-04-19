use opencv::{
    core::{self, MatTraitConst},
    highgui, imgcodecs, imgproc::{self, THRESH_OTSU}, Result,
};
use std::sync::{Arc, Mutex};

pub fn main() -> Result<()> {
    morphology()?;
    Ok(())
}
fn threshold() -> Result<()> {
    let img = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face.jpg",
        imgcodecs::IMREAD_COLOR,
    )?));
    let img_clone = Arc::clone(&img);
    highgui::imshow("dst", &*img.lock().unwrap()).unwrap();

    highgui::named_window("dst", highgui::WINDOW_AUTOSIZE)?;
    highgui::create_trackbar(
        "Threshold",
        "dst",
        None,
        100,
        Some(Box::new({
            move |val| {
                let img_guard = img_clone.lock().unwrap();
                let mut dst = core::Mat::default();
                imgproc::threshold(
                    &*img_guard,
                    &mut dst,
                    val as f64,
                    255.,
                    imgproc::THRESH_BINARY,
                )
                .unwrap();
                highgui::imshow("dst", &dst).unwrap();
            }
        })),
    )?;
    highgui::set_trackbar_pos("Threshold", "dst", 128)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn adaptive() -> Result<()> {
    let img = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face.jpg",
        imgcodecs::IMREAD_GRAYSCALE,
    )?));
    let img_clone = Arc::clone(&img);
    highgui::imshow("dst", &*img.lock().unwrap()).unwrap();

    highgui::named_window("dst", highgui::WINDOW_AUTOSIZE)?;
    highgui::create_trackbar(
        "Threshold",
        "dst",
        Some(&mut 0),
        200,
        Some(Box::new({
            move |val| {
                let img_guard = img_clone.lock().unwrap();

                let mut bsize = val;
                if bsize % 2 == 0 {
                    bsize -= 1;
                }
                if bsize<3 {
                    bsize=3;
                }

                let mut dst: opencv::prelude::Mat = core::Mat::default();
                imgproc::adaptive_threshold(&*img_guard, &mut dst, 255., imgproc::ADAPTIVE_THRESH_GAUSSIAN_C, imgproc::THRESH_BINARY, bsize, 0.).unwrap();
                highgui::imshow("dst", &dst).unwrap();
            }
        })),
    )?;
    highgui::set_trackbar_pos("Threshold", "dst", 11)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn morphology() -> Result<()> {
    let img = imgcodecs::imread(
        "./img/face.jpg",
        imgcodecs::IMREAD_GRAYSCALE,
    )?;
    highgui::imshow("dst", &img).unwrap();

    highgui::named_window("dst", highgui::WINDOW_AUTOSIZE)?;
    let mut bin= core::Mat::default();
    imgproc::threshold(
        &img,
        &mut bin,
        0 as f64,
        255.,
        imgproc::THRESH_BINARY|THRESH_OTSU,
    )
    .unwrap();
    
    let mut dst1= core::Mat::default();
    let mut dst2= core::Mat::default();
    

    imgproc::erode(&bin, &mut dst1, &core::Mat::default(), core::Point_::default(), 0, 0,core::Scalar::default())?;
    imgproc::dilate(&bin, &mut dst2, &core::Mat::default(), core::Point_::default(), 0, 0,core::Scalar::default())?;

    highgui::imshow("img", &img).unwrap();
    highgui::imshow("bin", &bin).unwrap();
    highgui::imshow("dst1", &dst1).unwrap();
    highgui::imshow("dst2", &dst2).unwrap();

    
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
