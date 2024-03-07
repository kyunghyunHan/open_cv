use anyhow::Result;
use opencv::{
    core::{self, Mat, Vector}, highgui::{self, imshow}, imgcodecs
};

pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    // highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &source_img)?;
    highgui::wait_key(10000)?;
    let arguments: Vector<i32> = Vector::new();

    imgcodecs::imwrite("dd.png", &source_img, &arguments)?;
    Ok(())
}