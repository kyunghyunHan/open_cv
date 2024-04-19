use opencv::{
    core::{self, MatTraitConst},
    highgui, imgcodecs, imgproc, Result,
};
/*영상의 명암비 조절 */
pub fn main() -> Result<()> {
    contrast2()?;
    Ok(())
}

fn contrast2() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    if src.empty() {
        println!("Error");
        std::process::exit(0);
    }

    let alpha = 1.;
    let beta = -128.0 * alpha;
    let mut dst = core::Mat::default();
    core::add(
        &src,
        &core::Scalar::all(beta),
        &mut dst,
        &core::no_array(),
        -1,
    )?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
