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

    let img2 = Mat::roi(&img1, rect).unwrap().try_clone().unwrap();
    let img3 = Mat::roi(&img1, rect)?.clone_pointee();

  
    
    let mut img2 = Mat::default();

    bitwise_not(&img1, &mut img2, &no_array()).unwrap();

    imshow("img1", &img1)?;
    imshow("img2", &img2)?;
    imshow("img3", &img3)?;
    wait_key(0)?;

    Ok(())
}
