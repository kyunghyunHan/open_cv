/* Mat op */

use opencv::{
    core::{bitwise_not, no_array, Mat, MatTrait, MatTraitConst, Rect, Scalar},
    highgui::{imshow, wait_key,destroy_all_windows},
    imgcodecs::{imread, IMREAD_COLOR},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;
fn mat_op1() -> Result<()> {
    let mut img1 = imread("./img/face1.jpeg", IMREAD_COLOR)?;
    if img1.empty() {
        println!("{}", "image load faild!");
        std::process::exit(0);
    }
    let img2 = img1.clone();
    img1.set_to(&Scalar::from((0, 255, 255)), &no_array())?;
    let img3 = &img1;
    imshow("img1", &img1)?;
    imshow("img2", &img2)?;
    imshow("img3", &img3)?;
    wait_key(0)?;

    Ok(())
}

fn mat_op2() -> Result<()> {
    let img1 = Arc::new(Mutex::new(imread("./img/bike0.png", 1)?));
    if img1.lock().unwrap().empty() {
        println!("{}", "image load faild!");
        std::process::exit(0);
    }
    let img1_clone = Arc::clone(&img1);

    let img2 = &img1_clone;
    let img3;
    img3 = &img1_clone;

    let img4 = img1_clone.lock().unwrap().clone();
    let mut img5 = Mat::default();
    img1_clone.lock().unwrap().copy_to(&mut img5).unwrap();
    img1.lock()
        .unwrap()
        .set_to(&Scalar::from((0, 255, 255)), &no_array())?;
    imshow("img1", &*img1.lock().unwrap())?;
    imshow("img2", &*img2.lock().unwrap())?;
    imshow("img3", &*img3.lock().unwrap())?;
    imshow("img4", &img4)?;
    imshow("img5", &img5)?;

    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}

pub fn main() -> Result<()> {
    // mat_op1()?;

    mat_op2()?;
    Ok(())
}
