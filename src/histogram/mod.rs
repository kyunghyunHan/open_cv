/*
histogram


영상의 히스토그램이란 영상의 픽셀 값 분포를 그래프 형태로 표현한 것을 의미합니다.그레이 스케일 영상의 경우 그레이스케일 값에 해달하는 픽셀의 개수를 구하고 이를 막대 그래프 형태로 표현함으로써 히스토그램을 구할수 있습니다.
*/

use opencv::{
    core::{
        divide2, min_max_loc, multiply, no_array, sub_scalar_mat, subtract, Mat, MatExprTraitConst,
        MatTraitConst, Point, Point_, Scalar, Vector, CV_32F, CV_32FC1, CV_8U, CV_8UC1,
    },
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::{calc_hist, calc_hist_def, equalize_hist, line, LINE_AA},
    Result,
};

fn calc_gray_hist(img: &Mat) -> Result<Mat> {
    // 입력 이미지가 CV_8UC1인지 확인
    // assert_eq!(img.typ(), CV_8UC1);
    let mut imgs: Vector<Mat> = Vector::new();
    imgs.push(img.clone());
    // 히스토그램을 저장할 빈 Mat 생성 (0으로 초기화, CV_32F 형식으로 설정)
    let mut hist = Mat::default();

    // 그레이스케일 이미지의 채널 (0은 유일한 채널)
    let channels = Vector::from_slice(&[0]);

    // 히스토그램의 크기 (그레이스케일의 경우 256개 빈)
    let hist_size = Vector::from_slice(&[256]);

    // 그레이스케일의 범위 (0부터 255까지)
    let ranges = Vector::from_slice(&[0_f32, 255_f32]);

    // 히스 토그램 계산
    calc_hist(
        &imgs,
        &channels,
        &no_array(),
        &mut hist,
        &hist_size,
        &ranges,
        false,
    )?;
    Ok(hist)
}

fn get_gray_hist_image(hist: &Mat) -> Result<Mat> {
    let mut hist_max = 0.;
    println!("{}", 1);
    min_max_loc(
        hist,
        Some(&mut 0.),
        Some(&mut hist_max),
        Some(&mut Point::default()),
        Some(&mut Point::default()),
        &no_array(),
    )?;
    let mut img_hist = Mat::new_rows_cols_with_default(100, 256, CV_8UC1, Scalar::all(255.))?;
    println!("{}", hist_max);
    for i in 0..256 {
        // 히스토그램 값 읽기
        let hist_value = hist.at_2d::<f32>(i, 0)?; // `f32` 타입으로 읽기

        // 정수로 변환 (히스토그램 최대 값에 대한 비율로 변환)
        let height = (100.0 - (hist_value * 100.0) / hist_max as f32).round() as i32;

        // 그래프의 선 그리기
        line(
            &mut img_hist,
            Point::new(i, 100),
            Point::new(i, height),
            Scalar::all(0.),
            2,
            LINE_AA,
            0,
        )?;
    }
    println!("{}", hist_max);

    Ok((img_hist))
}
fn b() -> Result<()> {
    let src = imread("./img/camera.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("error");
    }
    println!("{:?}", src);
    println!("{:?}", src.size());
    let hist = calc_gray_hist(&src)?;
    let hist_img = get_gray_hist_image(&hist)?;
    imshow("src", &src)?;
    imshow("hist_img", &hist_img)?;

    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn histogram() -> Result<()> {
    let src = imread("./img/hawkes.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("error");
    }

    let (mut gmin, mut gmax) = (0., 0.);

    min_max_loc(
        &src,
        Some(&mut gmin),
        Some(&mut gmax),
        Some(&mut Point::default()),
        Some(&mut Point::default()),
        &no_array(),
    )?;
    //Mat dst = (src -gmin) * 255/(gmax-gmin)
    let mut dst = Mat::default();
    let mut dst_sub = Mat::default();
    let mut dst_mul = Mat::default();

    subtract(&src, &gmin, &mut dst_sub, &no_array(), -1)?;
    multiply(
        &dst_sub,
        &Scalar::all(255.0 / (gmax - gmin)),
        &mut dst,
        1.0,
        -1,
    )?;
    let src_hist = calc_gray_hist(&src)?;
    let src_hist_img = get_gray_hist_image(&src_hist)?;

    let dst_hist = calc_gray_hist(&dst)?;
    let dst_hist_img = get_gray_hist_image(&dst_hist)?;
    imshow("src", &src)?;
    imshow("srcHist", &src_hist_img)?;
    imshow("dst", &dst)?;
    imshow("dstHist", &dst_hist_img)?;

    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn histogram_equzlization() -> Result<()> {
    let src = imread("./img/hawkes.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("error");
    }

    let mut dst = Mat::default();
    equalize_hist(&src, &mut dst)?;

    let src_hist = calc_gray_hist(&src)?;
    let src_hist_img = get_gray_hist_image(&src_hist)?;

    let dst_hist = calc_gray_hist(&dst)?;
    let dst_hist_img = get_gray_hist_image(&dst_hist)?;
    imshow("src", &src)?;
    imshow("srcHist", &src_hist_img)?;
    imshow("dst", &dst)?;
    imshow("dstHist", &dst_hist_img)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
pub fn main() -> Result<()> {
    // b()?;
    // histogram()?;
    histogram_equzlization()?;
    Ok(())
}
