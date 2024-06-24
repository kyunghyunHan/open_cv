use opencv::{
    core::{Mat, MatTraitConst, Rect},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc, Result,
    highgui::{imshow,wait_key}
};

pub fn main() -> Result<()> {
    let img1 = imread("./img/lion.jpeg", IMREAD_COLOR)?;
    if img1.empty() {
        println!("{}", "image load faild!");
        std::process::exit(0);
    }
    let rect = Rect::new(110,60, 100, 50);
    let img2 = img1.roi(rect)?;
    imshow("./aa", &img2)?;
    wait_key(0)?;
    println!("{}", 1);

    Ok(())
}
