use opencv::{
    core::{self, MatTraitConst}, highgui, imgcodecs::{self, IMREAD_COLOR}, imgproc, Result
};

pub fn main() -> Result<()> {
    let img = imgcodecs::imread("./img/face.jpg", IMREAD_COLOR)?;
    if img.empty() {
        std::process::exit(0);
    }

    highgui::imshow("Image Display", &img)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
