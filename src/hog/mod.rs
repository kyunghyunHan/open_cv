use opencv::prelude::VideoCaptureTrait;
use opencv::{
    core::{self, MatTraitConst},
    highgui, imgcodecs, imgproc,
    objdetect::{self, HOGDescriptorTrait, HOGDescriptorTraitConst},
    videoio::{self, VideoCaptureTraitConst},
    Result,
};
use rand::Rng;
pub fn main() -> Result<()> {
    let mut cap = videoio::VideoCapture::from_file("./video/vtest.avi", 0)?;
    if !cap.is_opened()? {
        println!("{}", "video open failed");
        std::process::exit(0);
    }
    let mut rng = rand::thread_rng();

    let mut hog = objdetect::HOGDescriptor::default()?;

    hog.set_svm_detector(&objdetect::HOGDescriptor::get_default_people_detector()?)?;

    let mut frame = core::Mat::default();

    loop {
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }
        let mut detected: core::Vector<core::Rect> = core::Vector::new();
        hog.detect_multi_scale(
            &frame,
            &mut detected,
            0.0,                   // hit_threshold
            core::Size::new(8, 8), // win_stride
            core::Size::new(0, 0), // padding
            1.05,                  // scale
            2.0,                   // final_threshold
            false,                 // use_meanshift_grouping
        )?;
        println!("Detected pedestrians: {}", detected.len()); // 검출된 보행자 수 출력

        for r in detected {
            let c = core::Scalar::from((
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256),
            ));
            imgproc::rectangle(&mut frame, r, c, 3, imgproc::LINE_AA, 0)?;
        }

        highgui::imshow("frame", &frame)?;
        if highgui::wait_key(10)? == 27 {
            break;
        }
    }

    Ok(())
}
