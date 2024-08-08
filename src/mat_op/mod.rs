use opencv::{
    core::{bitwise_not, no_array, Mat, MatTraitConst,Rect},
    highgui::{imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    Result,
};

pub fn main() -> Result<()> {
    let img1 = imread("./img/face1.jpeg", IMREAD_COLOR)?;
    if img1.empty() {
        println!("{}", "image load faild!");
        std::process::exit(0);
    }
    let rect = Rect::new(150, 150, 150, 150);

    let img2 = Mat::roi(&img1, rect)?;
    let img3 = Mat::roi(&img1, rect)?.clone_pointee();

  
    
    let mut img2_not = Mat::default();

    bitwise_not(&img2, &mut img2_not, &no_array()).unwrap();

    imshow("img1", &img1)?;
    imshow("img2", &img2_not)?;
    imshow("img3", &img3)?;
    wait_key(0)?;

    Ok(())
}
