/*
histogram

*/

use opencv::{
    core::{no_array, Mat, MatExprTraitConst, Vector, CV_32FC1},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::calc_hist,
    Result,
};
fn calc_gray_hist(img: Mat) -> Result<Mat> {
    // let mut hist = Mat::default();
    let mut hist = Mat::zeros(1, 256, CV_32FC1)?.to_mat()?;

    let channels: Vector<i32> = Vector::from_slice(&[0]);
    let dims = 1;
    let hist_size: Vector<i32> = Vector::from_slice(&[256]);
    let gray_level = vec![0., 256.];
    let ranges = Vector::from_slice(&gray_level);

    calc_hist(
        &img,
        &channels,
        &no_array(),
        &mut hist,
        &hist_size,
        &ranges,
        false,
    )?;
    Ok(hist)
}
fn calc_gray_hist_fn() -> Result<()> {
    let mut src = imread("./img/lenna.bmp", IMREAD_GRAYSCALE)?;
    let hist = calc_gray_hist(src)?;

    imshow("src", &hist)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}
pub fn main() -> Result<()> {
    calc_gray_hist_fn()?;
    Ok(())
}
