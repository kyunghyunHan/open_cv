use opencv::{
    core::{self, MatTraitConst, Point_, Scalar, Size_},
    highgui, imgcodecs,
    imgproc::{self, get_rotation_matrix_2d, FONT_HERSHEY_SIMPLEX, INTER_CUBIC, INTER_LANCZOS4, INTER_NEAREST},
    Result,
};
use std::os::raw::c_void;
pub fn main() -> Result<()> {
    // affine_scale()?;
    affine_file()?;
    Ok(())
}
fn affine_translation() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }
    let m = unsafe {
        core::Mat::new_rows_cols_with_data(
            2,                                                                // 행 수
            3,                                                                // 열 수
            core::CV_64FC1,                                                   // 데이터 타입
            &[1.0, 0.0, 100.0, 0.0, 1.0, 100.0] as *const f64 as *mut c_void, // 데이터의 포인터
            core::Mat_AUTO_STEP, // 행렬의 간격 (자동으로 계산됨)
        )?
    };
    let mut dst = core::Mat::default();

    imgproc::warp_affine(
        &src,
        &mut dst,
        &m,
        Size_::default(),
        imgcodecs::IMREAD_COLOR,
        0,
        Scalar::from(0),
    )?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn affine_shear() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }
    let m = unsafe {
        core::Mat::new_rows_cols_with_data(
            2,                                                            // 행 수
            3,                                                            // 열 수
            core::CV_64FC1,                                               // 데이터 타입
            &[1.0, 0.3, 0.0, 0.0, 1.0, 0.0] as *const f64 as *mut c_void, // 데이터의 포인터
            core::Mat_AUTO_STEP, // 행렬의 간격 (자동으로 계산됨)
        )?
    };
    let mut dst = core::Mat::default();

    imgproc::warp_affine(
        &src,
        &mut dst,
        &m,
        Size_::from((
            (src.cols() as f32 + src.rows() as f32 * 0.3).round() as i32,
            src.rows(),
        )),
        imgcodecs::IMREAD_COLOR,
        0,
        Scalar::from(0),
    )?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn affine_scale() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }

    let mut dst1 = core::Mat::default();
    let mut dst2 = core::Mat::default();
    let mut dst3 = core::Mat::default();
    let mut dst4 = core::Mat::default();

    imgproc::resize(&src, &mut dst1, Size_::default(), 4., 4., INTER_NEAREST)?;
    imgproc::resize(&src, &mut dst2, Size_::from((1920, 1280)), 0., 0., 0)?;
    imgproc::resize(
        &src,
        &mut dst3,
        Size_::from((1920, 1280)),
        0.,
        0.,
        INTER_CUBIC,
    )?;
    imgproc::resize(
        &src,
        &mut dst4,
        Size_::from((1920, 1280)),
        0.,
        0.,
        INTER_LANCZOS4,
    )?;
    let dst1_roi = dst1.roi(core::Rect::new(400, 500, 400, 400))?;
    let dst2_roi = dst1.roi(core::Rect::new(400, 500, 400, 400))?;
    let dst3_roi = dst1.roi(core::Rect::new(400, 500, 400, 400))?;
    let dst4_roi = dst1.roi(core::Rect::new(400, 500, 400, 400))?;
    highgui::imshow("src", &src)?;
    highgui::imshow("dst1", &dst1_roi)?;
    highgui::imshow("dst2", &dst2_roi)?;
    highgui::imshow("dst3", &dst3_roi)?;
    highgui::imshow("dst4", &dst4_roi)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn affine_rotaion() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }

    let cp = core::Point2f::new(src.cols() as f32 / 2., src.rows() as f32 / 2.);
    let m = get_rotation_matrix_2d(cp, 20., 1.)?;

    let mut dst = core::Mat::default();
    imgproc::warp_affine(
        &src,
        &mut dst,
        &m,
        Size_::default(),
        imgcodecs::IMREAD_COLOR,
        0,
        Scalar::from(0),
    )?;
    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;

    Ok(())
}

fn affine_file() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }
    highgui::imshow("src", &src)?;
    let mut dst = core::Mat::default();

    let filp_code:core::Vector<i32> = core::Vector::from_iter([1,0,-1]);
    for i in 0..3{
       core::flip(&src, &mut dst, filp_code.get(i )?)?;

       let desc= format!("flipCode{}",filp_code.get(i)?);
       imgproc::put_text(&mut dst, &desc, Point_::from((10,30)), FONT_HERSHEY_SIMPLEX, 1., core::Scalar::from((255,0,0)), 1, imgproc::LINE_AA, false)?;
       highgui::imshow("dst", &dst)?;
       highgui::wait_key(0)?;

    }
    highgui::destroy_all_windows()?;

    Ok(())
}
