use std::{array::from_ref, default};

use opencv::{
    core::{self, no_array},
    highgui, imgcodecs, imgproc, Result,
};

pub fn main() -> Result<()> {
    otsu_algorithm()?;
    Ok(())
}
fn histogram() -> Result<()> {
    let img = imgcodecs::imread("./img/supper.png", imgcodecs::IMREAD_COLOR)?;
    let mut hist = core::Mat::default();

    let channels = core::Vector::from_iter([2]); // 흑백 이미지의 경우 채널 인덱스는 0
    let hist_size = core::Vector::from_iter([256]); // 0부터 255까지의 픽셀 값 범위
    let ranges = core::Vector::from_slice(&[0.0, 256.0]); // 픽셀 값의 범위

    imgproc::calc_hist(
        &img,
        &channels,
        &no_array(),
        &mut hist,
        &hist_size,
        &ranges,
        false,
    )?;

    highgui::imshow("hist", &hist)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;

    Ok(())
}
fn otsu_algorithm() -> Result<()> {
    let mut img= imgcodecs::imread("./img/pace.jpg", imgcodecs::IMREAD_COLOR)?;
    // let t(,bin_img)= imgproc::threshold(&img, dst, thresh, maxval, typ)?;
    Ok(())
}
