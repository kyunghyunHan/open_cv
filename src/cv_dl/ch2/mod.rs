use opencv::{
    core::{self, MatTraitConst},
    highgui::{self, WINDOW_AUTOSIZE},
    imgcodecs, imgproc,
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst},
    Result,
};
use std::sync::{Arc, Mutex};

pub fn main() -> Result<()> {
    mouse_rectangle()?;
    Ok(())
}

pub fn img_display() -> Result<()> {
    let img = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        std::process::exit(0);
    }

    highgui::imshow("Image Display", &img)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
pub fn size() -> Result<()> {
    let img = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        std::process::exit(0);
    }
    let mut dst = core::Mat::default();
    imgproc::cvt_color(&img, &mut dst, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut resize_dst = core::Mat::default();

    imgproc::resize(
        &dst,
        &mut resize_dst,
        core::Size_::from((0, 0)),
        0.5,
        0.5,
        0,
    )?;
    highgui::imshow("Image Display", &img)?;
    highgui::imshow("Image Display2", &resize_dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

pub fn video_capture() -> Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; //카메라 연결시도
    let opened = videoio::VideoCapture::is_opened(&cap)?;

    if !opened {
        println!("카메라 연결 실패");
        std::process::exit(0);
    }

    loop {
        let mut frame = core::Mat::default();
        let mut ret = core::Mat::default();
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;
        // cap.read(&mut frame)?;

        highgui::imshow("Video", &frame)?;
        let key = highgui::wait_key(1);
        if key? as u8 as char == 'q' {
            break;
        }
    }
    // cap.release(); //카메라와 연결을 끊음
    highgui::destroy_all_windows()?;
    Ok(())
}

pub fn video_capture_three() -> Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; //카메라 연결시도
    let opened = videoio::VideoCapture::is_opened(&cap)?;

    if !opened {
        println!("카메라 연결 실패");
        std::process::exit(0);
    }
    let mut frames: core::Vector<core::Mat> = core::Vector::new();
    loop {
        let mut frame = core::Mat::default();
        let mut ret = core::Mat::default();

        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;
        cap.read(&mut ret)?;

        if ret.empty() {
            println!("{}", "프레임에 실패하여 루프를 나갑니다.");
            break;
        }

        highgui::imshow("Video", &frame)?;
        let key = highgui::wait_key(1)?;
        if key as u8 as char == 'c' {
            frames.push(frame);
        } else if key as u8 as char == 'q' {
            break;
        }
    }
    // cap.release(); //카메라와 연결을 끊음

    cap.release();
    highgui::destroy_all_windows()?;
    if frames.len() > 0 {
        let mut imgs = frames.get(0)?.clone();
        let mut img_refs: core::Vector<core::Mat> = core::Vector::new(); // Create a vector of references
        img_refs.push(imgs); // Add the first image as reference
        for i in 1..std::cmp::min(3, frames.len()) {
            img_refs.push(frames.get(i)?); // Add references to other images
        }
        let mut img = core::Mat::default();
        core::hconcat(&img_refs, &mut img)?; // Use hconcat with references
        imgs = img;
        highgui::imshow("Video", &imgs)?;
    }
    Ok(())
}
fn text_rectangle() -> Result<()> {
    let mut img = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }

    imgproc::rectangle(
        &mut img,
        core::Rect_::from((415, 30, 500, 200)),
        core::Scalar::from((0, 0, 255)),
        2,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::put_text(
        &mut img,
        "laugh",
        core::Point_::from((830, 24)),
        imgproc::FONT_HERSHEY_SIMPLEX,
        1.,
        core::Scalar::from((255, 0, 0)),
        2,
        imgproc::LINE_AA,
        false,
    )?;

    highgui::imshow("Draw", &img)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
/*마우스를 클릭한 곳에 직사각형 그리기 */

fn mouse_rectangle() -> Result<()> {
    let mut img: Arc<Mutex<core::Mat>> = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face.jpg",
        imgcodecs::IMREAD_COLOR,
    )?));

    if img.lock().unwrap().empty() {
        println!("image load failed");
        std::process::exit(0);
    }
    let img_clone = Arc::clone(&img);

    highgui::named_window("Drawing", WINDOW_AUTOSIZE)?;
    highgui::imshow("Drawing", &*img_clone.lock().unwrap())?;
    highgui::set_mouse_callback(
        "Drawing",
        Some(Box::new({
            move |event, x, y, _flag| {
                let mut img_guard = img_clone.lock().unwrap();
                println!("1");

                if event == highgui::EVENT_LBUTTONDOWN {
                    println!("1");
                    imgproc::rectangle(
                        &mut *img_guard,
                        core::Rect::from((x, y, x + 200, y + 200)),
                        core::Scalar::from((0, 0, 255)),
                        2,
                        imgproc::LINE_AA,
                        0,
                    )
                    .unwrap()
                } else if event == highgui::EVENT_RBUTTONDOWN {
                    println!("1");
                    imgproc::rectangle(
                        &mut *img_guard,
                        core::Rect::from((x, y, x + 100, y + 100)),
                        core::Scalar::from((255, 0, 0)),
                        2,
                        imgproc::LINE_AA,
                        0,
                    )
                    .unwrap()
                }

                highgui::imshow("Drawing", &*img_guard).unwrap();
            }
        })),
    )?;

    loop {
        if highgui::wait_key(1)? as u8 as char == 'q' {
            highgui::destroy_all_windows()?;
            break;
        }
    }
    Ok(())
}
