use anyhow::Result;
use opencv::{
    core::{self, Mat, Vector},
    highgui::{self, imshow},
    imgcodecs,
};

pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    highgui::imshow("hello opencv!", &source_img)?;

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
