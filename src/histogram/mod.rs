/*
histogram


영상의 히스토그램이란 영상의 픽셀 값 분포를 그래프 형태로 표현한 것을 의미합니다.그레이 스케일 영상의 경우 그레이스케일 값에 해달하는 픽셀의 개수를 구하고 이를 막대 그래프 형태로 표현함으로써 히스토그램을 구할수 있습니다.
컬러영상에 대해서도 세개의 색상 성분 조합 에 따른 픽셀 개수를 계산하여 히스토그램을 구할수 잇습니다.

왼쪽 그림은 4x4행렬의 입력 영상은 각 픽셀이 0부터 7사이의 밝기를 가질수 있는 단순한 형태입니다. 이영상에서 값이 0인 픽셀개수를 세어보면 4이고,밝기 값이 1인 픽셀개수는 3입니다.
이처럼 각각의 밝기에 해당하는 픽셀 개수를 세어서 막대그래프 형태로 표현한 히스토그램이 오른쪽 그림입니다.

히스토그램 그래프에서 가로축을 히르소그램의 bin 이라 하며 왼쪽 그림에서 나타난 히스토그램에서 bin 개수는 8입니다.사용된 영상이 0부터 7사이의 픽셀값을 가질수 있기 때문에 여덟개의 빈으로 구성된 히스토 그램을 생성한 것입니다.
그레이스케일 영상의 경우 256개의 빈을 갖는 히스토그램을 구하는 것이 일반적입니다.

그러나 히스토그램의 빈개수가 항상 여덟개의  픽셀 값 범위와 같아야 하는 것은 아닙니다.경우에 다라서 더 작게 설정 할수 있습니다.
예를 들어 여덟개의 밝기 값을 가질수 잇는 영상에서 히스토그램의 빈 개수를 4로 설정이 가능하며 이러한 히스토그램은 다음과 같이 나타낼수 있습니다.

opencv에서 히스토그램을 구하려면 calcHist()함수를 사용해야합니다.이 함수는 한장의 영상뿐만 아니라 여러장의 영상으로부터 히스토그램을 구할수 있고 여러 채널로부터 히스토그램을 구할수 있습니다.또한 히스토그램 빈 개수도 설정이 가능합니다.


calcHist

- images:입력 영상의 배열 또는 입력 영상의 주소,영상의 배열인 경우 모든 영상의 크기와 깊이는 같아야함
- numages:입력 영상 개수
- channels:히스토그램을 구할 채널을 나타내는 정수형 배열
- mask:마스크 영상.입력 영상과 같은 8비트 배열이어야하며, 마스크 행렬의 원소 값이 0이 아닌 좌표의 픽셀만 히스토그램 계산에 사용
- hist 출력 히스토그램 CV_32F깊이를 사용하는 dums-차원의 행렬
- dims:출력 히스토그램의 차원수
- histSize:각 차원의 히스토그램 배열 크기를 나타내는 배열(즉 각 차원의 히스토그램 빈 개수를 나타내는 배열)
- ranges : 각 차원의 히스토그램 범위
- uniform:히스토그램 빈의 간격이 균등한지를 나타내는 플러그
- accumulate:누적 플러그 ,이값이 True이면 hist배열을 초기화 하지않고 누적하여 히스토그램 계산

하나의 그레이 스케일 로부터 히스토그램을 구하는 코드

calc_gray_hist()함수는 내부에서 calc_hist()함수를 이용하여 그레이스케일 영상의 히스토그램을 표현하는 행렬 hist를 구하여 반환합니다.
이떄 반환하는 hist는 CV_32FC1타입을 갖는 256x1크기의 행렬입니다.즉 hist행렬의 행 개수는 256 열은 1 입니다
calc_gray_hist()함수로 구한 행렬을 막대그래프로 나타내려면 hist행렬을 참조하여 막대 그래프영상을 생성해야합니다 
hist행렬로부터 가로가 256픽셀 세로가 100픽셀인 크기의 히스토그램 영상을 생성할수 잇습니다.



히스토그램 스트레칭

히스토그램 스트레칭은 영상의 히스토그램이 그레이스케일 전 구간에 걸쳐서 나타나다도록 변경하는 선형 변환 기법입니다.
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
fn test() -> Result<()> {
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
    test()?;
    // histogram()?;
    // histogram_equzlization()?;
    Ok(())
}
