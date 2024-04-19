use opencv::{
    core::{self, no_array, MatTraitConst, Point_, Scalar},
    highgui, imgcodecs, imgproc, Result,
};

pub fn main() -> Result<()> {
    hough_circles()?;
    Ok(())
}
/*hought_lines함수의 첫번쨰 인자 Imaged에는 보통 canny함수 등을 이용하여 구한 에지 영상 지정 */
fn hought_lines() -> Result<()> {
    let src = imgcodecs::imread("./img/pp.png", imgcodecs::IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }

    let mut edge = core::Mat::default();
    imgproc::canny(&src, &mut edge, 50., 150., 3, false)?;

    let mut lines: core::Vector<core::Vec2f> = core::Vector::new();

    imgproc::hough_lines(
        &edge,
        &mut lines,
        1.,
        core::CV_PI / 180.,
        100,
        0.,
        0.,
        0.,
        0.,
    )?;
    println!("{}", lines.len());

    let mut dst = core::Mat::default();
    imgproc::cvt_color(&edge, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;
    for i in 0..lines.len() {
        println!("{}", 1);
        let r = *lines.get(i)?.get(0).unwrap();
        let t = *lines.get(i)?.get(1).unwrap();
        let cos_t = t.cos();
        let sin_t = t.sin();
        let x0 = r * cos_t;
        let y0 = r * sin_t;
        let alpha = 1000.;

        let pt1 = core::Point_::from((
            (x0 + alpha * -sin_t).round() as i32,
            (y0 + alpha * cos_t).round() as i32,
        ));
        let pt2 = core::Point_::from((
            (x0 - alpha * -sin_t).round() as i32,
            (y0 - alpha * cos_t).round() as i32,
        ));
        println!("{}", 1);
        imgproc::line(
            &mut dst,
            pt1,
            pt2,
            Scalar::from((0, 0, 255)),
            2,
            imgproc::LINE_AA,
            0,
        )?;
    }
    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
/*확률적  허프 변환*/
fn hough_line_segments() -> Result<()> {
    let src = imgcodecs::imread("./img/pp.png", imgcodecs::IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }

    let mut edge = core::Mat::default();
    imgproc::canny(&src, &mut edge, 50., 150., 3, false)?;

    let mut lines: core::Vector<core::Vec4i> = core::Vector::new();

    imgproc::hough_lines_p(&edge, &mut lines, 1., core::CV_PI / 180., 160, 50., 5.)?;
    println!("{}", lines.len());

    let mut dst = core::Mat::default();
    imgproc::cvt_color(&edge, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;
    for i in lines {
        imgproc::line(
            &mut dst,
            Point_::from((*i.get(0).unwrap(), *i.get(1).unwrap())),
            Point_::from((*i.get(2).unwrap(), *i.get(3).unwrap())),
            Scalar::from((0, 0, 255)),
            2,
            imgproc::LINE_AA,
            0,
        )?;
    }
    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
fn hough_circles() -> Result<()> {
    let src = imgcodecs::imread("./img/cir.png", imgcodecs::IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "Image load failed");
        std::process::exit(0);
    }
    let mut blurred = core::Mat::default();
    imgproc::blur(
        &src,
        &mut blurred,
        core::Size::from((3, 3)),
        Point_::default(),
        0,
    )?;
    let mut circles: core::Vector<core::Vec3f> = core::Vector::new();
    imgproc::hough_circles(
        &blurred,
        &mut circles,
        imgproc::HOUGH_GRADIENT,
        1.,
        50.,
        250.,
        30.,
        0,
        0,
    )?;
    let mut dst = core::Mat::default();
    imgproc::cvt_color(&src, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;

    for c in circles {
        let center = core::Point::from((*c.get(0).unwrap() as i32, *c.get(1).unwrap() as i32));
        let radius = *c.get(2).unwrap();
        imgproc::circle(&mut dst, center, radius as i32, Scalar::from((0,0,255)), 2, imgproc::LINE_AA, 0)?;
    }
    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
