use opencv::{
    Result,
    core::{self, Mat, Vector}, highgui::{self, imshow}, imgcodecs,    imgproc,

};

pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    // highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &source_img)?;
    highgui::wait_key(10000)?;
    Ok(())
}