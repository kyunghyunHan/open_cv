use opencv::{
    core::{self, MatTraitConst, Size_}, highgui, imgcodecs::{self, IMREAD_COLOR}, imgproc::{self, COLOR_BGR2GRAY}, Result
};

pub fn main() -> Result<()> {
    let img = imgcodecs::imread("./img/face.jpg", IMREAD_COLOR)?;
    if img.empty() {
        std::process::exit(0);
    }
    let mut dst= core::Mat::default();
    imgproc::cvt_color(&img, &mut dst, COLOR_BGR2GRAY, 0)?;
    let mut resize_dst= core::Mat::default();

    imgproc::resize(&dst, &mut resize_dst, Size_::from((0,0)), 0.5, 0.5,0 )?;
    highgui::imshow("Image Display", &img)?;
    highgui::imshow("Image Display2", &resize_dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
