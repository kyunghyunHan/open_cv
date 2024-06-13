use opencv::{core::Mat, Result,imgcodecs::imread,imgproc};

pub fn main() -> Result<()> {
    let area = 0;
    let cnt = 0;
    let img = Mat::default();
    let img_result = Mat::default();
    let img_gray = Mat::default();

    let img_input =  imread("scan.jpg", 1)?;

    Ok(())
}
