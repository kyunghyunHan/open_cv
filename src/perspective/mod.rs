use opencv::{
    core::{self, MatTraitConst, Scalar, Size_},
    highgui::{self, EVENT_LBUTTONDOWN},
    imgcodecs,
    imgproc::{self, get_perspective_transform},
    Result,
};
use std::sync::{Arc, Mutex};

pub fn main() -> Result<()> {
    perspective()?;
    Ok(())
}
fn perspective() -> Result<()> {
    let mut src = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face.jpg",
        imgcodecs::IMREAD_COLOR,
    )?));
    if src.lock().unwrap().empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }
    let img_clone = Arc::clone(&src);

    highgui::named_window("src", highgui::WINDOW_AUTOSIZE)?;
    let cnt = Arc::new(Mutex::new(0)); // cnt를 공유 상태로 만듦
    let mut a: core::Vector<core::Point2f> = core::Vector::new();
    let mut b: core::Vector<core::Point2f> = core::Vector::new();
    highgui::set_mouse_callback(
        "src",
        Some(Box::new(move |event, x, y, _flag| {
            let mut img_guard = img_clone.lock().unwrap();
            let mut cnt_guard = cnt.lock().unwrap(); // cnt에 대한 잠금 획득
            if event == EVENT_LBUTTONDOWN {
                if *cnt_guard < 4 {
                    a.push(core::Point2f::new(x as f32, y as f32));
                    *cnt_guard += 1; // cnt 증가
                    imgproc::circle(
                        &mut *img_guard,
                        core::Point_::from((x, y)),
                        5,
                        Scalar::from((0, 0, 255)),
                        -1,
                        imgproc::LINE_AA,
                        0,
                    )
                    .unwrap();
                    highgui::imshow("src", &*img_guard).unwrap();
                    if *cnt_guard == 4 {
                        let w = 200;
                        let h = 300;
                        b.push(core::Point2f::new(0.0, 0.0));
                        b.push(core::Point2f::new((w - 1) as f32, 0.0));
                        b.push(core::Point2f::new((w - 1) as f32, (h - 1) as f32));
                        b.push(core::Point2f::new(0.0, (h - 1) as f32));
                        let pers = get_perspective_transform(&a, &b, 1).unwrap();

                        let mut dst = core::Mat::default();
                        imgproc::warp_perspective(
                            &*img_guard,
                            &mut dst,
                            &pers,
                            Size_::from((w, h)),
                            imgcodecs::IMREAD_COLOR,
                            0,
                            Scalar::from(0),
                        )
                        .unwrap();
                        highgui::imshow("dst", &dst).unwrap();
                    }
                }
            }
        })),
    )?;
    highgui::imshow("src", &*src.lock().unwrap()).unwrap();

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
