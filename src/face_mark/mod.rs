use opencv::{core::{Point2f, Rect, Size_, Vector,Point}, face::FacemarkLBF, highgui, imgproc::{cvt_color, COLOR_BGR2GRAY,polylines,circle}, objdetect::CascadeClassifier, prelude::*, videoio, Result};
pub fn main() -> Result<()> {

    let mut face_detector:CascadeClassifier = CascadeClassifier::new("./dataset/haarcascade_frontalface_alt2.xml").unwrap();
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
        let mut gray= Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
         let mut faces :Vector<Rect>= Vector::new();
         cvt_color(&mut frame, &mut gray, COLOR_BGR2GRAY, 0).unwrap();
         face_detector.detect_multi_scale(&mut gray, &mut faces, 1.1, 0, 0 ,Size_::default(), Size_::default()).unwrap();
         let mut landmarks :Vector<Vector<Point2f>>= Vector::default();
        let success :bool = facemark.fit(&mut frame, &faces, &mut landmarks).unwrap();

        if success{
            for i in 0..landmarks.len(){
                println!("{}",1);
            }
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
// fn draw_polyline(
//     im: &mut Mat,
//     landmarks: &Vector<Point>,
//     start: i32,
//     end: i32,
//     is_closed: bool,
// ) -> opencv::Result<()> {
//     let points: Vector<Point> = landmarks.get(start as usize..=end as usize).unwrap().into();
//     polylines(im, &points, is_closed, COLOR, 2, opencv::imgproc::LINE_16, 0)?;
//     Ok(())
// }
fn draw_landmarks(im: &mut Mat, landmarks: &Vector<Point>) -> opencv::Result<()> {
    if landmarks.len() == 68 {
        // draw_polyline(im, &landmarks, 0, 16, false)?;   // Jaw line
        // draw_polyline(im, &landmarks, 17, 21, false)?;  // Left eyebrow
        // draw_polyline(im, &landmarks, 22, 26, false)?;  // Right eyebrow
        // draw_polyline(im, &landmarks, 27, 30, false)?;  // Nose bridge
        // draw_polyline(im, &landmarks, 30, 35, true)?;   // Lower nose
        // draw_polyline(im, &landmarks, 36, 41, true)?;   // Left eye
        // draw_polyline(im, &landmarks, 42, 47, true)?;   // Right eye
        // draw_polyline(im, &landmarks, 48, 59, true)?;   // Outer lip
        // draw_polyline(im, &landmarks, 60, 67, true)?;   // Inner lip
    } else {
        for point in landmarks {
            circle(im, *point, 3, COLOR, opencv::imgproc::FILLED, opencv::imgproc::LINE_8, 0)?;
        }
    }
    Ok(())
}
// hog.detect_multi_scale(
//     &frame,
//     &mut detected,
//     0.0,                   // hit_threshold
//     core::Size::new(8, 8), // win_stride
//     core::Size::new(0, 0), // padding
//     1.05,                  // scale
//     2.0,                   // final_threshold
//     false,                 // use_meanshift_grouping
// )?;