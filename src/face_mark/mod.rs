use opencv::{
    core::{Point, Point2f, Rect, Scalar, Size_, Vector},
    face::FacemarkLBF,
    highgui,
    imgproc::{
        circle, cvt_color, polylines, put_text, COLOR_BGR2GRAY, FONT_HERSHEY_SIMPLEX, LINE_8,
    },
    objdetect::CascadeClassifier,
    prelude::*,
    videoio, Result,
};

pub fn main() -> Result<()> {
    let mut face_detector: CascadeClassifier =
        CascadeClassifier::new("./dataset/haarcascade_frontalface_alt2.xml").unwrap();
    let mut facemark = FacemarkLBF::create_def().unwrap();
    facemark.load_model("./dataset/lbfmodel.yaml").unwrap();
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::from_file("./video/smile1.mov", 0)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open video file!");
    }
    loop {
        let mut frame = Mat::default();
        let mut gray = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
        let mut faces: Vector<Rect> = Vector::new();
        cvt_color(&mut frame, &mut gray, COLOR_BGR2GRAY, 0).unwrap();
        face_detector
            .detect_multi_scale(
                &mut gray,
                &mut faces,
                1.1,
                3,
                0,
                Size_::default(),
                Size_::default(),
            )
            .unwrap();

        if faces.len() > 0 {
            println!("Faces detected: {}", faces.len());
        }

        let mut landmarks: Vector<Vector<Point2f>> = Vector::default();
        let success: bool = facemark.fit(&mut frame, &faces, &mut landmarks).unwrap();

        if success {
            println!("Landmarks detected for {} faces", landmarks.len());
            for i in 0..landmarks.len() {
                println!("{:?}", &landmarks.get(i).unwrap());
                draw_landmarks(&mut frame, &landmarks.get(i).unwrap()).unwrap();
            }
        }

        highgui::imshow(window, &frame)?; // 프레임을 업데이트하여 그린 랜드마크를 표시
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn draw_polyline(
    im: &mut Mat,
    landmarks: &Vector<Point2f>,
    start: i32,
    end: i32,
    is_closed: bool,
) -> opencv::Result<()> {
    let mut points: Vector<Point> = Vector::new();
    for i in start..=end {
        let point = landmarks.get(i as usize).unwrap();
        points.push(Point::new(point.x.round() as i32, point.y.round() as i32));
        // 좌표를 반올림하여 정수형으로 변환
    }

    polylines(
        im,
        &points,
        is_closed,
        Scalar::from((0, 0, 255)),
        1,
        LINE_8,
        0,
    )?;
    Ok(())
}

fn draw_landmarks(im: &mut Mat, landmarks: &Vector<Point2f>) -> opencv::Result<()> {
    if landmarks.len() != 68 {
        println!("Drawing landmarks with 68 points");
        draw_polyline(im, &landmarks, 0, 16, false)?;   // Jaw line
        draw_polyline(im, &landmarks, 17, 21, false)?;  // Left eyebrow
        draw_polyline(im, &landmarks, 22, 26, false)?;  // Right eyebrow
        draw_polyline(im, &landmarks, 27, 30, false)?;  // Nose bridge
        draw_polyline(im, &landmarks, 30, 35, true)?;   // Lower nose
        draw_polyline(im, &landmarks, 36, 41, true)?;   // Left eye
        draw_polyline(im, &landmarks, 42, 47, true)?;   // Right eye
        draw_polyline(im, &landmarks, 48, 59, true)?;   // Outer lip
        draw_polyline(im, &landmarks, 60, 67, true)?;   // Inner lip
    } else {
        for (i, point) in landmarks.iter().enumerate() {
            let color = if i == 48 || i == 54 {
                Scalar::from((0, 255, 0)) // Green for corners of the mouth
            } else {
                Scalar::from((0, 0, 255)) // Red for other points
            };

            circle(
                im,
                Point::new(point.x.round() as i32, point.y.round() as i32), // 좌표를 반올림하여 정수형으로 변환
                3,
                color,
                0,
                opencv::imgproc::LINE_8,
                0,
            )?;
            put_text(
                im,
                &i.to_string(),
                Point::new(point.x.round() as i32, point.y.round() as i32),
                FONT_HERSHEY_SIMPLEX,
                0.4,
                color,
                1,
                LINE_8,
                false,
            )?;
        }
    }
    Ok(())
}