use opencv::{
    core::{bitwise_not, no_array, Mat, MatTrait, MatTraitConst, Rect, Scalar},
    highgui::{imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    Result,
};
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

pub fn main() -> Result<()> {
    // mat_op1()?;

    //mat_op2()?;
    Ok(())
}
