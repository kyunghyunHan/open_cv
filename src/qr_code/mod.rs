use opencv::{
    core::{self, no_array, MatTraitConst},
    highgui, imgcodecs, imgproc,
    objdetect::{self, GraphicalCodeDetectorTraitConst},
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst},
    Result,
};

pub fn main() -> Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if !cap.is_opened()? {
        println!("{}", "Camera open failed");
        std::process::exit(0);
    }

    let detector = objdetect::QRCodeDetector::default()?;

    let mut frame = core::Mat::default();

    loop {
        cap.read(&mut frame)?;

        if frame.empty() {
            println!("{}", "frame load failed");
            break;
        }

        let mut point: core::Vector<core::Point> = core::Vector::new();

        let info = detector.detect_and_decode(&frame, &mut point, &mut no_array())?;
        if !info.is_empty() {
            let string = String::from_utf8(info).unwrap();

            imgproc::polylines(
                &mut frame,
                &point,
                true,
                core::Scalar::from((0, 0, 255)),
                2,
                imgproc::LINE_AA,
                0,
            )?;
            imgproc::put_text(
                &mut frame,
                &string,
                core::Point_::from((10, 30)),
                imgproc::FONT_HERSHEY_DUPLEX,
                1.,
                core::Scalar::from((0, 0, 255)),
                2,
                imgproc::LINE_AA,
                false,
            )?;
        }

        highgui::imshow("frame", &frame)?;
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
