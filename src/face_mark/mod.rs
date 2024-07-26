use opencv::{face::FacemarkLBF, highgui, objdetect::CascadeClassifier, prelude::*, videoio, Result};
pub fn main() -> Result<()> {

    let face_detector:CascadeClassifier = CascadeClassifier::new("haarcascade_frontalface_alt2.xml").unwrap();
    let mut facemark = FacemarkLBF::create_def().unwrap();
    facemark.load_model("./dataset/lbfmodel.yaml").unwrap();
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::from_file("./video/girl.mp4", 0)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {

        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
