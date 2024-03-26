use opencv::imgproc::INTER_AREA;
use opencv::{
    core, highgui,
    imgproc::{resize, resize_def},
    prelude::*,
    videoio, Result,
};
const SCLAR: f32 = 0.3;
pub fn main() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let xml = core::find_file_def("shape_predictor_68_face_landmarks.dat")?;
    println!("{}", xml);
    let mut cam = videoio::VideoCapture::from_file("girl.mp4", 0)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        let width = frame.size()?.width / 3;
        let height = frame.size()?.height / 3;
        //resize
        let mut resized_frame = Mat::default();
        resize(
            &frame,
            &mut resized_frame,
            core::Size { width, height },
            0.3f64,
            0.3f64,
            INTER_AREA,
        )
        .expect("Can't resize image");
        if frame.size()?.width > 0 {
            highgui::imshow(window, &resized_frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
