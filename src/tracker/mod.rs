use opencv::imgproc::rectangle;
use opencv::tracking::{TrackerKCF, TrackerKCFTrait, TrackerKCF_Params};
use opencv::video::TrackerTrait;
use opencv::{core, highgui, prelude::*, videoio, Result};

pub fn main() -> Result<()> {
    // let window = "video capture";
    // highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
 
    let mut cam =
        videoio::VideoCapture::from_file("./video/human.mp4", videoio::CAP_FFMPEG)?; // 0 is the default camera

    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut param: opencv::tracking::TrackerKCF_Params =
        opencv::tracking::TrackerKCF_Params::default()?;

    let mut t = TrackerKCF::create(param)?;


    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    let mut bounding_box = opencv::highgui::select_roi("1", &mut frame, true, true, false).unwrap();

    t.init(&mut frame, bounding_box)?;

   
    loop {
        cam.read(&mut frame)?;
        if frame.rows() == 0 || frame.cols() == 0{
            break;
        }
        t.update(&mut frame, &mut bounding_box)?;

        rectangle(
            &mut frame,
            bounding_box,
            core::Scalar::new(0f64, -1f64, -1f64, -1f64),
            2,
            8,
            0,
        )?;

        highgui::imshow("1", &mut frame)?;

        //			highgui::imshow(window, &mut frame)?;
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
  