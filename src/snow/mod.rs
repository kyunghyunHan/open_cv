use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, videoio, Result};
use std::collections::VecDeque;
#[derive(Debug)]
struct Points {
    faces: VecDeque<core::Point_<i32>>,
    centers: (i32, i32),
}

impl Points {
    fn new(centers: (i32, i32)) -> Self {
        Points {
            faces: VecDeque::from([core::Point_ {
                x: (centers.0 + centers.1 / 2),
                y: 300,
            }]),
            centers,
        }
    }

    fn get_point(
        &mut self,
        faces: core::Vector<core::Rect_<i32>>,
    ) -> (core::Point_<i32>, core::Point_<i32>) {
        let last_face = self.faces.back().unwrap().x;
        let closest = faces.iter().reduce(|accum, f| {
            let dx = (accum.x + accum.width / 2 - last_face).abs();
            if dx <= (f.x + f.width / 2 - last_face).abs() {
                accum
            } else {
                f
            }
        });

        if let Some(f) = closest {
            self.faces.push_back(core::Point_ {
                x: f.x + f.width / 2,
                y: f.y + f.height / 2,
            });
        }

        if self.faces.len() > 3 {
            self.faces.pop_front();
        }

        let mut tx = 0;
        let mut ty = 0;

        for f in self.faces.iter() {
            tx += f.x;
            ty += f.y;
        }
        tx = tx / self.faces.len() as i32;
        ty = ty / self.faces.len() as i32;

        let dx = (tx - 700) as f32;
        let dy = (ty - 300) as f32;
        let px = 50.0 * (dy.atan2(dx)).cos();
        let py = 50.0 * (dy.atan2(dx)).sin();

        (
            core::Point_ {
                x: px as i32 + self.centers.0,
                y: py as i32 + 300,
            },
            core::Point_ {
                x: px as i32 + self.centers.1,
                y: py as i32 + 300,
            },
        )
    }
}

pub fn main() -> Result<()> {
    let window = "Stalker";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Error opening default camera");
    }
	let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let mut face = objdetect::CascadeClassifier::new(&xml)?;

    // let mut face = objdetect::CascadeClassifier::new("/haarcascades/haarcascade_frontalface_default.xml")?;
    // let mut side = objdetect::CascadeClassifier::new(
    //     "/usr/local/Cellar/opencv/4.6.0/share/opencv4/haarcascades/haarcascade_profileface.xml",
    // )?;

    let mut f = Points::new((400, 800));
    let mut img = Mat::default();

    loop {
        cam.read(&mut img)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut faces = types::VectorOfRect::new();
        face.detect_multi_scale(
            &gray,
            &mut faces,
            1.1,
            10,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size::new(10, 10),
            core::Size::new(0, 0),
        )?;
        // side.detect_multi_scale(
        //     &gray,
        //     &mut faces,
        //     1.1,
        //     10,
        //     objdetect::CASCADE_SCALE_IMAGE,
        //     core::Size::new(10, 10),
        //     core::Size::new(0, 0),
        // )?;

        imgproc::rectangle(
            &mut img,
            core::Rect {
                x: 0,
                y: 0,
                width: 2000,
                height: 2000,
            },
            core::Scalar::new(255f64, 255f64, 255f64, 255f64),
            -1,
            8,
            0,
        )?;

        // Eyebrows
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 400, y: 270 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            180.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            5,
            1,
            0,
        )?;
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 800, y: 270 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            180.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            5,
            1,
            0,
        )?;

        // Iris
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 800, y: 300 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            0.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            -1,
            1,
            0,
        )?;
        imgproc::ellipse(
            &mut img,
            core::Point_ { x: 400, y: 300 },
            core::Size_ {
                width: 150,
                height: 75,
            },
            0.0,
            0.0,
            360.0,
            core::Scalar::new(0f64, 0f64, 0f64, 100f64),
            -1,
            imgproc::FILLED,
            0,
        )?;

        let points = f.get_point(faces);

        imgproc::circle(
            &mut img,
            points.0,
            30,
            core::Scalar::new(200f64, 50f64, 0f64, 0f64),
            -1,
            imgproc::FILLED,
            0,
        )?;
        imgproc::circle(
            &mut img,
            points.1,
            30,
            core::Scalar::new(200f64, 50f64, 0f64, 0f64),
            -1,
            8,
            0,
        )?;

        let mut flipped = img.clone();
        core::flip(&img, &mut flipped, 1)?;
        // imgproc::put_text(
        //     &mut flipped,
        //     &format!("faces: {}", faces.len()),
        //     core::Point_ { x: 10, y: 80 },
        //     0,
        //     3.0,
        //     core::Scalar::new(0f64, 0f64, 0f64, 0f64),
        //     3,
        //     3,
        //     false,
        // )?;

        highgui::imshow(window, &flipped)?;

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}