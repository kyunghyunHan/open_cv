use opencv::{
    core::{self, MatTraitConst, Scalar},
    highgui, imgcodecs, imgproc, Result,
};

pub fn main() -> Result<()> {
    canny_edge()?;
    Ok(())
}

fn canny_edge() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_GRAYSCALE)?;
    if src.empty() {
        println!("{}", "load failed");
        std::process::exit(0);
    }

    let mut dst1 = core::Mat::default();
    let mut dst2 = core::Mat::default();

    imgproc::canny(&src, &mut dst1, 50., 100., 3, false)?;
    imgproc::canny(&src, &mut dst2, 50., 150., 3, false)?;

    // 이미지 반전
    let inverted_src = invert_image(&dst1)?;

    highgui::imshow("src", &inverted_src)?;
    highgui::imshow("dst1", &dst1)?;
    highgui::imshow("dst2", &dst2)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;

    Ok(())
}

fn invert_image(src: &core::Mat) -> Result<core::Mat> {
    let mut inverted_src = core::Mat::default();
    core::bitwise_not(&src, &mut inverted_src, &core::no_array())?;
    Ok(inverted_src)
}

