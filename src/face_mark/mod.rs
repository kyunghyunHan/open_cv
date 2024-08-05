use opencv::{
    core::{Point, Point2f, Rect, Rect_, Scalar, Size_, Vec3b, Vec4b, Vector},
    face::FacemarkLBF,
    highgui,
    imgcodecs::{imread, IMREAD_UNCHANGED},
    imgproc::{
        circle, cvt_color, polylines, put_text, resize, COLOR_BGR2GRAY, FONT_HERSHEY_SIMPLEX,
        LINE_8,
    },
    objdetect::CascadeClassifier,
    prelude::*,
    videoio, Result,
};

pub fn main() -> Result<()> {
    // let logo: Mat = imread("./img/perfect.png", IMREAD_UNCHANGED).unwrap();
    // println!("{:?}", logo);
    let mut face_detector: CascadeClassifier =
        CascadeClassifier::new("./dataset/haarcascade_frontalface_alt2.xml").unwrap();
    let mut facemark = FacemarkLBF::create_def().unwrap();
    facemark.load_model("./dataset/lbfmodel.yaml").unwrap();
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::from_file("./video/face1.mp4", 0)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open video file!");
    }
    let frame_width = cam.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let frame_height = cam.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    // let logo_width = frame_width / 4; // 로고 크기를 동영상의 1/4로 조정
    // let logo_height = (logo.rows() * logo_width) / logo.cols();
    // let mut logot = Mat::default();
    // resize(
    //     &logo,
    //     &mut logot,
    //     Size_::new(logo_width, logo_height),
    //     0.0,
    //     0.0,
    //     1,
    // )?;

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
            // println!("Faces detected: {}", faces.len());
        }
        let x_offset = frame_width / 2; // 오른쪽 하단에 로고 배치
        let y_offset = frame_height / 2;
        // let roi = Rect::new(x_offset, y_offset, logot.cols(), logot.rows());

        // overlay_image(&mut frame, &logot, roi);

        let mut landmarks: Vector<Vector<Point2f>> = Vector::default();
        let success: bool = facemark.fit(&mut frame, &faces, &mut landmarks).unwrap();

        if success {
            // println!("Landmarks detected for {} faces", landmarks.len());
            for i in 0..landmarks.len() {
                draw_landmarks(&mut frame, &landmarks.get(i).unwrap()).unwrap();
            }
        }

        highgui::imshow(window, &frame)?; // 프레임을 업데이트하여 그린 랜드마크를 표시
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            // println!("{:?}",landmarks.get(0).unwrap().get(0).unwrap());

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
fn overlay_image(frame: &mut Mat, logo: &Mat, roi: Rect) {
    for y in 0..logo.rows() {
        for x in 0..logo.cols() {
            let frame_pixel = frame.at_2d_mut::<Vec3b>(y + roi.y, x + roi.x).unwrap();
            let logo_pixel = logo.at_2d::<Vec4b>(y, x).unwrap();
            let alpha = logo_pixel[3] as f32 / 255.0;
            for c in 0..3 {
                frame_pixel[c] =
                    (alpha * logo_pixel[c] as f32 + (1.0 - alpha) * frame_pixel[c] as f32) as u8;
            }
        }
    }
}
fn draw_landmarks(
    im: &mut Mat,
    landmarks: &Vector<Point2f>,
    // log: &Mat,
    // roi: Rect_<i32>,
) -> opencv::Result<()> {
    if landmarks.len() != 68 {
        println!("Drawing landmarks with 68 points");
        draw_polyline(im, &landmarks, 0, 16, false)?; // Jaw line
        draw_polyline(im, &landmarks, 17, 21, false)?; // Left eyebrow
        draw_polyline(im, &landmarks, 22, 26, false)?; // Right eyebrow
        draw_polyline(im, &landmarks, 27, 30, false)?; // Nose bridge
        draw_polyline(im, &landmarks, 30, 35, true)?; // Lower nose
        draw_polyline(im, &landmarks, 36, 41, true)?; // Left eye
        draw_polyline(im, &landmarks, 42, 47, true)?; // Right eye
        draw_polyline(im, &landmarks, 48, 59, true)?; // Outer lip
        draw_polyline(im, &landmarks, 60, 67, true)?; // Inner lip
    } else {
        /*
        왼쪽 끝 48번
        오른 쪽 끝 54번

        가운데 위 51번
        아래


        최대길이 180

        최소길이
         */
        let middle_top_mouse = landmarks.get(51).unwrap();
        let middle_buttom_mouse = landmarks.get(57).unwrap();

        let left_mouse = landmarks.get(48).unwrap();
        let right_mouse = landmarks.get(54).unwrap();

        println!("위  - 아래{}", middle_buttom_mouse.y - middle_top_mouse.y);
        println!("오른쪽  - 왼쪾{}", right_mouse.x - left_mouse.x);

        circle(
            im,
            Point::new(
                middle_top_mouse.x.round() as i32,
                middle_top_mouse.y.round() as i32,
            ), // 좌표를 반올림하여 정수형으로 변환
            4,
            Scalar::from((0, 0, 255)),
            1,
            opencv::imgproc::LINE_8,
            0,
        )?;
        circle(
            im,
            Point::new(
                middle_buttom_mouse.x.round() as i32,
                middle_buttom_mouse.y.round() as i32,
            ), // 좌표를 반올림하여 정수형으로 변환
            4,
            Scalar::from((0, 0, 255)),
            1,
            opencv::imgproc::LINE_8,
            0,
        )?;
        circle(
            im,
            Point::new(left_mouse.x.round() as i32, left_mouse.y.round() as i32), // 좌표를 반올림하여 정수형으로 변환
            4,
            Scalar::from((0, 0, 255)),
            1,
            opencv::imgproc::LINE_8,
            0,
        )?;
        circle(
            im,
            Point::new(right_mouse.x.round() as i32, right_mouse.y.round() as i32), // 좌표를 반올림하여 정수형으로 변환
            3,
            Scalar::from((0, 0, 255)),
            1,
            opencv::imgproc::LINE_8,
            0,
        )?;

        // println!(
        //     "{}",
        //     right_mouse.x.round() as i32 - left_mouse.x.round() as i32
        // );
        /*  a */
        if middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32 > 80
            && right_mouse.x.round() as i32 - left_mouse.x.round() as i32 > 100
        {
            println!("이거다");
            // overlay_image( im, log, roi);
            put_text(
                im,
                &"A",
                Point::new(200, 200), // 좌표를 반올림하여 정수형으로 변환
                FONT_HERSHEY_SIMPLEX,
                2.0,
                Scalar::from((0, 255, 255)),
                3,
                LINE_8,
                false,
            )?;
            /*  e */
        } else if middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32 > 70
            && right_mouse.x.round() as i32 - left_mouse.x.round() as i32 > 120
        {
            // println!(
            //     "{}",
            //     right_mouse.x.round() as i32 - left_mouse.x.round() as i32
            // );
            // overlay_image( im, log, roi);
            put_text(
                im,
                &"E",
                Point::new(200, 200), // 좌표를 반올림하여 정수형으로 변환
                FONT_HERSHEY_SIMPLEX,
                2.0,
                Scalar::from((0, 255, 255)),
                3,
                LINE_8,
                false,
            )?;
            /*  i */
        } else if 60 > middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32
            && right_mouse.x.round() as i32 - left_mouse.x.round() as i32 > 130
        {
            println!(
                "{}",
                right_mouse.x.round() as i32 - left_mouse.x.round() as i32
            );
            // overlay_image( im, log, roi);
            put_text(
                im,
                &"I",
                Point::new(200, 200), // 좌표를 반올림하여 정수형으로 변환
                FONT_HERSHEY_SIMPLEX,
                2.0,
                Scalar::from((0, 255, 255)),
                3,
                LINE_8,
                false,
            )?;
            /*  o */
        } else if 60 > middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32
            && 40 < middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32
            && right_mouse.x.round() as i32 - left_mouse.x.round() as i32 > 70
        {
            println!(
                "{}",
                right_mouse.x.round() as i32 - left_mouse.x.round() as i32
            );
            // overlay_image( im, log, roi);
            put_text(
                im,
                &"O",
                Point::new(200, 200), // 좌표를 반올림하여 정수형으로 변환
                FONT_HERSHEY_SIMPLEX,
                2.0,
                Scalar::from((0, 255, 255)),
                3,
                LINE_8,
                false,
            )?;
            /*  u */
        } else if 40 > middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32
            && 30 < middle_buttom_mouse.y as i32 - middle_top_mouse.y as i32
            && right_mouse.x.round() as i32 - left_mouse.x.round() as i32 > 70
        {
            println!(
                "{}",
                right_mouse.x.round() as i32 - left_mouse.x.round() as i32
            );
            // overlay_image( im, log, roi);
            put_text(
                im,
                &"U",
                Point::new(200, 200), // 좌표를 반올림하여 정수형으로 변환
                FONT_HERSHEY_SIMPLEX,
                2.0,
                Scalar::from((0, 255, 255)),
                3,
                LINE_8,
                false,
            )?;
        }
        // println!("{:?}",test);
        // for (i, point) in landmarks.iter().enumerate() {
        //     let color = if i == 48 || i == 54 {
        //         Scalar::from((0, 255, 0)) // Green for corners of the mouth
        //     } else {
        //         Scalar::from((0, 0, 255)) // Red for other points
        //     };

        //     if i > 47 && i < 60 {
        //         circle(
        //             im,
        //             Point::new(point.x.round() as i32, point.y.round() as i32), // 좌표를 반올림하여 정수형으로 변환
        //             3,
        //             color,
        //             0,
        //             opencv::imgproc::LINE_8,
        //             0,
        //         )?;

        //         //48부터 60까지
        //     }

        //     let first_point  = landmarks.get(48).unwrap();
        //     let second_poist = landmarks.get(54).unwrap();

        //     if i == 48 || i == 54 {
        //         println!("{:?}",first_point);
        //         println!("{:?}",second_poist);
        //         println!("point is here {:?}", point);
        //         count+=point.x;
        //         println!("{}", count);

        //         // put_text(
        //         //     im,
        //         //     &"good",
        //         //     Point::new(point.x.round() as i32, point.y.round() as i32),
        //         //     FONT_HERSHEY_SIMPLEX,
        //         //     0.4,
        //         //     color,
        //         //     1,
        //         //     LINE_8,
        //         //     false,
        //         // )?;
        //     }
        // }
    }
    Ok(())
}
