use opencv::{
    core::{MatTrait, MatTraitConst, Scalar},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    Result,
};

fn mask_set_to() -> Result<()> {
    let mut src = imread("./img/lenna.bmp", IMREAD_COLOR)?;
    let mask = imread("./img/mask_smile.bmp", IMREAD_GRAYSCALE)?;

    if src.empty() || mask.empty() {
        panic!("image load faild")
    }
    src.set_to(&Scalar::from((0, 255, 255)), &mask)?;

    imshow("src", &src)?;
    imshow("mask", &mask)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
fn mask_copy_to() -> Result<()> {
    let  src = imread("./img/airplane.bmp", IMREAD_COLOR)?;
    let mask = imread("./img/mask_plane.bmp", IMREAD_GRAYSCALE)?;
    let mut dst = imread("./img/field.bmp", IMREAD_COLOR)?;
    if src.empty() || mask.empty() || dst.empty() {
        panic!("image load faild")
    }
    src.copy_to_masked(&mut dst, &mask)?;
    imshow("src", &dst)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}
pub fn main() -> Result<()> {
    // mask_set_to()?;
    mask_copy_to()?;
    Ok(())
}
