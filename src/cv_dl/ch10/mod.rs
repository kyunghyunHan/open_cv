use opencv::{
    core, dnn, highgui, imgcodecs,
    imgproc::{self, cvt_color},
    prelude::VideoCaptureTrait,
    videoio, Result,
};
pub fn main() -> Result<()> {
    klt_algorithm()?;
    Ok(())
}
/*추적 */
fn klt_algorithm() -> Result<()> {
    let mut cap =
        videoio::VideoCapture::from_file("./video/slow_traffic_small.mp4", videoio::CAP_ANY)?;
    let feature_parame = 0;

    let (mut ret, mut old_frame) = (core::Mat::default(), core::Mat::default());

    cap.read(&mut ret)?;
    cap.read(&mut old_frame)?;


    

    loop {
        let mut frame = core::Mat::default();
        let (mut ret, mut frame) = (core::Mat::default(), core::Mat::default());
        let mut new_gray =core::Mat::default();
        cvt_color(&frame, &mut new_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    }
    Ok(())
}
