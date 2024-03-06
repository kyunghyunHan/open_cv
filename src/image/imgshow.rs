use anyhow::Result;
use opencv::{
    core::{self, Mat, Vector},
    imgcodecs,
};

pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;

    // Flipping image horizontally
    let mut destination_arr = Mat::default();
    core::flip(&source_img, &mut destination_arr, 1)?;

    // Creating an output image
    let arguments: Vector<i32> = Vector::new();
    imgcodecs::imwrite("final-output.png", &destination_arr, &arguments)?;
    Ok(())
}