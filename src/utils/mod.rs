use opencv::{
    core::{Mat, MatTrait, MatTraitConst, Scalar, TickMeter, TickMeterTrait, TickMeterTraitConst},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    Result,
};
/*
유용한 기능


마스크 연산
opencv에서는 임의의 모양을 갖는 ROI설정을 위하여 일부 행렬 연산 함수를 위하여 마스크 연산을 지원
마스크 연산을 지원하는 opencv 함수는 보통 입력 영상과 크기가 같고 깊이가 CV_8U인 마스크 영상을 함꼐 인자로 받는다.
마스크 영상이 주어질 경우 마스크 영상의 픽셀값이 0 이 아닌 좌표에 대해서만 연산이 수행된다.

set_to()는 마스크 연산을 지원하는 함수이며 두번쨰 인자에 mask영상을 지정할수 있다.기본값으로 설정되어 있는 no_array()를 mask인자로 지정하면 입력 행렬의 모든 원소값을 value값으로 설정하고
적절한 마스크 영상을 mask 인자로 지정하면 특정 영상에서만 픽셀 값을 설정 가능하다.이떄 마스크영상은 set_to()를 호출하는 대상 행렬과 크기가 같아야 한다.

예시를 보면 픽셀값이 0이 아닌 위치에서만 src 영상 픽셀을 노란색으로 설정한것이 보인다.

copy_to는 복사할 대상 행렬과 마스크 영상 두개를 인자로 받는다.mask영상의 픽셀값이 0이 아닌 위치에서만 *this행렬 원소값을 행렬 m으로 복사한다.
src에서 비행기가 위치에서만 픽셀값이 255이고 나머지는 픽셀값이 0이다.dst영상은 들판영상이며 src의 비행기부분만 copy되어 dst에 저장된걸 확인할수 있다.

연산 시간 측정
대부분의 영상 처리 시스템은 대용량 영상 데이터를 다루고 복잡한 알고리즘 연산을 수행하기 떄문에 각 단계에서 소요되는 연산 시간을 측정하고 시간이 오래 걸리는 부분을 찾아 개선하는 시스템 최적화 작업이 필수적이다.
특히 머신 비전 분야에서 처럼 실시간 연산을 필요로하는 시스템의 경우 매우 중요하다고 볼수 있다.
*/
fn mask_set_to() -> Result<()> {
    let mut src = imread("./img/lenna.bmp", IMREAD_COLOR)?;
    let mask = imread("./img/mask_smile.bmp", IMREAD_GRAYSCALE)?;

    if src.empty() || mask.empty() {
        panic!("image load faild")
    }
    src.set_to(&Scalar::from((0, 255, 255)), &mask)?;
    
    imshow("src", &src)?;
    imshow("mask", &mask)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
fn mask_copy_to() -> Result<()> {
    let src = imread("./img/airplane.bmp", IMREAD_COLOR)?;
    let mask = imread("./img/mask_plane.bmp", IMREAD_GRAYSCALE)?;
    let mut dst = imread("./img/field.bmp", IMREAD_COLOR)?;
    if src.empty() || mask.empty() || dst.empty() {
        panic!("image load faild")
    }
    src.copy_to_masked(&mut dst, &mask)?;
    imshow("src", &dst)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn time_inverse() -> Result<()> {
    let mut src = imread("./img/airplane.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("image load faild")
    }

    let mut dst = unsafe { Mat::new_rows_cols(src.rows(), src.cols(), src.typ()) }?;
    let mut tm: TickMeter = TickMeter::default()?;
    tm.start()?;
    for j in 0..src.rows() {
        for i in 0..src.cols() {
            *dst.at_2d_mut::<u8>(j, i)? = 255 - *src.at_2d_mut::<u8>(j, i)?;
        }
    }
    tm.stop()?;
    println!("image inverse took{:?}",tm.get_time_milli());
    Ok(())
}
pub fn main() -> Result<()> {
    mask_set_to()?;
    // mask_copy_to()?;
    // time_inverse()?;
    Ok(())
}
