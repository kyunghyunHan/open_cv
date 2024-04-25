use opencv::{
    core::{self, no_array, MatTraitConst},
    features2d::{self, DescriptorMatcherTrait, Feature2DTrait},
    highgui, imgcodecs, imgproc, Result,
};

pub fn main() -> Result<()> {
    Ok(())
}

fn keypoint_matching() -> Result<()> {
    let src1 = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_GRAYSCALE)?;
    let src2 = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_GRAYSCALE)?;

    if src1.empty() || src2.empty() {
        println!("Image load failed!");
        std::process::exit(0);
    }

    let mut feature = features2d::ORB::create_def()?;
    let mut keypoints1: core::Vector<core::KeyPoint> = core::Vector::new();
    let mut keypoints2: core::Vector<core::KeyPoint> = core::Vector::new();

    let mut desc1 = core::Mat::default();
    let mut desc2 = core::Mat::default();

    feature.detect_and_compute(&src1, &no_array(), &mut keypoints1, &mut desc1, false)?;
    feature.detect_and_compute(&src2, &no_array(), &mut keypoints2, &mut desc1, false)?;

    let mut matcher = features2d::BFMatcher::create(core::NORM_HAMMING, false)?;

    let mut matches: core::Vector<core::DMatch> = core::Vector::new();
    matcher.match_(&desc1, &mut matches, &desc2)?;

    let mut dst = core::Mat::default();
    // features2d::draw_matches(
    //     &src1,
    //     &keypoints1,
    //     &src2,
    //     &keypoints2,
    //     &mut matches,
    //     &mut dst,
    //     core::Scalar::default(),
    //     core::Scalar::default(),
    //     &no_array(),
    //     &no_array(),
    // )?;

    Ok(())
}
