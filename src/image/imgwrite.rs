use opencv::{
    Result,
    core::{self, Mat, MatTraitConstManual, Vec3b, Vector, CV_8UC3},
    highgui::{self, imshow,},
    imgcodecs,
    imgproc,
    types,
};
use opencv::prelude::MatTraitConst;
pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    
    highgui::imshow("hello opencv!", &source_img)?;
    let test:Vec<Vec<Vec3b>>= source_img.to_vec_2d()?;

    println!("test{:?}",test);
    loop {
        let k = highgui::wait_key(0)?;
        if k == 27 {
            highgui::destroy_all_windows()?;
            break;
        } else if k as u8 as char == 's' {
            println!("Write");
            imgcodecs::imwrite("s.png", &source_img, &Vector::new())?;
        } else if k as u8 as char == 'a' {
            println!("End");
            highgui::destroy_all_windows()?;
            break;
        }
    }

    Ok(())
}

