/*
histogram


영상의 히스토그램이란 영상의 픽셀 값 분포를 그래프 형태로 표현한 것을 의미합니다.그레이 스케일 영상의 경우 그레이스케일 값에 해달하는 픽셀의 개수를 구하고 이를 막대 그래프 형태로 표현함으로써 히스토그램을 구할수 있습니다.
*/

use opencv::{
    core::{no_array, Mat, MatExprTraitConst, MatTraitConst, Vector, CV_32F, CV_32FC1, CV_8UC1},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::calc_hist,
    Result,
};

fn calc_gray_hist(img: &Mat) -> Result<Mat> {
    // 입력 이미지가 CV_8UC1인지 확인
    assert_eq!(img.typ(), 0);

    // 히스토그램을 저장할 빈 Mat 생성
    let mut hist = Mat::default();
    // let mut hist = Mat::zeros(1, 256, CV_32F)?.to_mat()?;

    // 그레이스케일 이미지의 채널 (0은 유일한 채널)
    let mut channels = Vector::new();
    channels.push(0);

    // 히스토그램의 크기 (그레이스케일의 경우 256개 빈)
    let mut hist_size = Vector::new();
    hist_size.push(1);
    hist_size.push(256);

    // 그레이스케일의 범위 (0부터 255까지)
    let mut gray_level = Vector::new();
    gray_level.push(0.0);
    gray_level.push(255.0);
    let ranges = Vector::from(gray_level);
    
    // 히스토그램 계산
    calc_hist(
        &img,
        &channels,
        &no_array(),
        &mut hist,
        &hist_size,
        &ranges,
        false,
    )?;

    Ok(hist)
}
fn calc_gray_hist_fn() -> Result<()> {
    let  src = imread("./img/camera.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("error");
    }
    println!("{:?}", src);
    println!("{:?}", src.size());

    let hist = calc_gray_hist(&src)?;

    imshow("src", &hist)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}
pub fn main() -> Result<()> {
    calc_gray_hist_fn()?;
    Ok(())
}
