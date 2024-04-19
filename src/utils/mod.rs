use opencv::{
    core::{
        no_array, Mat, MatTrait, MatTraitConst, Scalar, Size, TickMeter, TickMeterTrait,
        TickMeterTraitConst,
    },
    highgui::{self, destroy_all_windows},
    imgcodecs::{self, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc, Result,
};
use opencv::core::{CV_32SC1,CV_32SC3};

/*
setto- 마스트 영상을 지정가능
copyto-
*/

pub fn main() -> Result<()> {
    // mask_set_to()?;
    time_inverse()?;
    Ok(())
}
pub fn mask_set_to() -> Result<()> {
    let mut src = imgcodecs::imread("./img/face.jpg", IMREAD_COLOR)?;
    let mut mask = imgcodecs::imread("./img/mask.jpeg", IMREAD_GRAYSCALE)?;

    println!("{:?}", src.size());
    println!("{:?}", &mask.size());
    let mut resized_mask = Mat::default();

    let new_size = Size {
        width: src.cols(),
        height: src.rows(),
    };
    imgproc::resize(
        &mask,
        &mut resized_mask,
        new_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;

    src.set_to(&Scalar::from((0, 255, 255)), &resized_mask)?;

    highgui::imshow("src", &src)?;
    highgui::imshow("mask", &mask)?;
    highgui::wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

pub fn mask_copy_to() -> Result<()> {
    let mut src = imgcodecs::imread("./img/face.jpg", IMREAD_COLOR)?;
    let mut mask = imgcodecs::imread("./img/mask.jpeg", IMREAD_GRAYSCALE)?;

    println!("{:?}", src.size());
    println!("{:?}", &mask.size());
    let mut resized_mask = Mat::default();

    let new_size = Size {
        width: src.cols(),
        height: src.rows(),
    };
    imgproc::resize(
        &mask,
        &mut resized_mask,
        new_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;

    src.set_to(&Scalar::from((0, 255, 255)), &resized_mask)?;

    highgui::imshow("src", &src)?;
    highgui::imshow("mask", &mask)?;
    highgui::wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn time_inverse() -> Result<()> {
    let mut src = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;
    let mut dst= src.clone();
    let mut tm = TickMeter::default()?;
    tm.start()?;
 
    for j in 0..src.rows() {
        for i in 0..src.cols() {
            println!("{}", src.at_2d::<u8>(j, i)?);
            *dst.at_2d_mut::<u8>(j, i)? = 255 - *src.at_2d_mut::<u8>(j, i)?;
        }
    }

    tm.stop()?;
    println!("image inverse took {}ms", tm.get_time_milli()?);

    Ok(())
}
