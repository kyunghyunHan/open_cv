use opencv::{
    core::{self, normalize, MatTraitConst, Point_, Scalar, CV_8U, NORM_MINMAX}, highgui::{self, wait_key}, imgcodecs::{self, IMREAD_GRAYSCALE}, imgproc::{self, circle, corner_harris, cvt_color}, Result
};
/*영상의 명암비 조절 */
pub fn main() -> Result<()> {
    corners()?;
    Ok(())
}

fn corners() -> Result<()> {
    let src = imgcodecs::imread("./img/pp.png", IMREAD_GRAYSCALE)?;

    let mut harris = core::Mat::default();
    corner_harris(&src, &mut harris, 3, 3, 0.04, 1)?;

    let mut harris_norm = core::Mat::default();
    normalize(
        &harris,
        &mut harris_norm,
        0.0,
        255.0,
        NORM_MINMAX,
        core::CV_32FC1,
        &core::no_array(),
    )?;
    let mut dst = core::Mat::default();
    cvt_color(&src, &mut dst, imgproc::COLOR_GRAY2BGR, 0)?;
    

    for j in 1..harris.rows() - 1 {
        for i in 1..harris.cols() - 1 {
            if harris_norm.at_2d::<f32>(j, i)? > &120.0 {
                if harris_norm.at_2d::<f32>(j, i)?> harris_norm.at_2d::<f32>(j - 1, i)?
                    && harris_norm.at_2d::<f32>(j, i)? > harris_norm.at_2d::<f32>(j + 1, i)?
                    && harris_norm.at_2d::<f32>(j, i)? > harris_norm.at_2d::<f32>(j, i - 1)?
                    && harris_norm.at_2d::<f32>(j, i)? > harris_norm.at_2d::<f32>(j, i + 1)?
                {
                    circle(&mut dst, Point_::from((i,j)), 5, Scalar::from((0,0,255)), 2, imgproc::LINE_AA, 0)?;

                }
            }
        }
    }

    
    highgui::imshow("src", &src)?;
    highgui::imshow("harris_norm", &harris_norm)?;
    highgui::imshow("dst", &dst)?;
    wait_key(0)?;
    Ok(())
}
 