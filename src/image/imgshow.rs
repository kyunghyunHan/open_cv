
use opencv::{Result,core,highgui,imgcodecs,imgproc};
pub fn main() -> Result<()> {
    let src = imgcodecs::imread("./img/lion.png", imgcodecs::IMREAD_COLOR)?;
    highgui::imshow("hello opencv!", &src)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}