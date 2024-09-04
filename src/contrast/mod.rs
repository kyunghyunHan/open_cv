use opencv::{
    core::{self, MatExprTraitConst, MatTraitConst},
    highgui::{self, destroy_all_windows, imshow, wait_key},
    imgcodecs::{self, imread, IMREAD_GRAYSCALE},
    imgproc, Result,
};
/*영상의 명암비 조절

명암비란 영상에서 밝은 영역과 어두운 영역 사이에 드러나는 밝기 차이의 강도를 말하고 명암 대비 또는 contrast라고도 한다  영상이 전반적으로 어둡거나 또는 전반적으로 밝은 픽셀로만 구성된 경우 명암비가 낮다고 표현하며 반면애 밝은 영역과 어두운 영역이 골고루 섞여있는 영상은 명암비가 높다고 말한다
일반적으로 명암비가 낮은 영상은 객체간의 구분이 잘 되지 않아 전반적으로 흐릿하게 느껴지고 명암비가 높은 영상은 사물의 구분이 잘되며 선명한 느낌을 준다


*/

pub fn main() -> Result<()> {
    contrast()?;
    contrast2()?;
    Ok(())
}
fn contrast() -> Result<()> {
    let src = imread("./img/bike0.png", IMREAD_GRAYSCALE)?;

    let s = 2.;
    let dst = (s * &src).into_result()?.to_mat()?;

    imshow("src", &src)?;
    imshow("dst", &dst)?;

    wait_key(0)?;
    destroy_all_windows()?;

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
