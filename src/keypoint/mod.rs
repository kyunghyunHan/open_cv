use opencv::{
    core::{self, no_array, MatTraitConst},
    features2d::{self, Feature2DTrait},
    highgui, imgcodecs, imgproc, Result,
};

pub fn main() -> Result<()> {
    keypoint()?;
    Ok(())
}
fn keypoint() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_GRAYSCALE)?;

    if src.empty() {
        println!("image load failed!");
        std::process::exit(0);
    }
    let mut feature = features2d::ORB::create_def()?;
    let mut keypoint: core::Vector<core::KeyPoint> = core::Vector::new();
    feature.detect(&src, &mut keypoint, &no_array())?;
    let mut desc = core::Mat::default();

    feature.compute(&src, &mut keypoint, &mut desc)?;

    println!("keypoints.size{}", keypoint.len());
    println!("desc.size{:?}", desc.size());

    let mut dst = core::Mat::default();

    features2d::draw_keypoints(
        &src,
        &mut keypoint,
        &mut dst,
        core::Scalar::all(-1.),
        features2d::DrawMatchesFlags::DRAW_RICH_KEYPOINTS,
    )?;

    highgui::imshow("src", &src)?;
    highgui::imshow("dst", &dst)?;

    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}
