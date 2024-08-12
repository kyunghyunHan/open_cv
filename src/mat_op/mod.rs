/* Mat op */

use opencv::{
    prelude::{MatTraitConstManual,MatTraitManual},
    core::{bitwise_not, no_array, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Rect, Scalar},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;
fn mat_op1() -> Result<()> {
    let mut img1 = imread("./img/face1.jpeg", IMREAD_COLOR)?;
    if img1.empty() {
        println!("{}", "image load faild!");
        std::process::exit(0);
    }
    let img2 = img1.clone();
    img1.set_to(&Scalar::from((0, 255, 255)), &no_array())?;
    let img3 = &img1;
    imshow("img1", &img1)?;
    imshow("img2", &img2)?;
    imshow("img3", &img3)?;
    wait_key(0)?;

    Ok(())
}

fn mat_op2() -> Result<()> {
    //사진을 불러와서 img1에 저장
    //Arc:같은 데이터를 여러 참조에서 공유할수 있게 하면서 메모리 안정성을 보장
    //Mutex:동시에 여러 스레드가 Img1에 접근하지 못하게 하여 데이터 경합 문제 방지
    let img1 = Arc::new(Mutex::new(imread("./img/bike0.png", IMREAD_COLOR)?));
    //이미지가 있는지 확인 없으면 Panic
    if img1.lock().unwrap().empty() {
        panic!("{}", "image load faild!");
    }
    //img1의 복사본 영상 img2,img3생성(얕은복사)
    let img2 = &img1;
    let img3;
    img3 = &img1;
    //Mat::clone()과 Mat::copy_to를 사용하여 img1의 복사본 영상 영상 img4,img5생성(깊은 복사)
    let img4 = img1.lock().unwrap().clone();
    let mut img5 = Mat::default();
    img1.lock().unwrap().copy_to(&mut img5).unwrap();
    //img1영상의 모든 픽셀을 노란색으로 변경
    img1.lock()
        .unwrap()
        .set_to(&Scalar::from((0, 255, 255)), &no_array())?;
    //각 이미지 출력
    imshow("img1", &*img1.lock().unwrap())?;
    imshow("img2", &*img2.lock().unwrap())?;
    imshow("img3", &*img3.lock().unwrap())?;
    imshow("img4", &img4)?;
    imshow("img5", &img5)?;
    //img2,img3이 img1의 픽셀 데이터를 공휴하기 때문에 노란색으로 변하였으며 img4와 img5영상은 깊은 복사를 수행하였기 때문에 바이트 이미지가 그대로 남아있다.
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
// fn mat_op3() -> Result<()> {
//     let img1 = imread("./img/bike0.png", IMREAD_COLOR)?;
//     if img1.empty() {
//         panic!("image load failed");
//     }
//     let rect = Rect::new(220, 120, 340, 240);
//     let img2 = Mat::roi(&img1, rect)?;
//     let img3 = Mat::roi(&img1, rect)?.try_clone()?;
//     let mut img2_bn = Mat::default();
//     // img2를 반전
//     bitwise_not(&img2, &mut img2_bn, &Mat::default())?;
  

//     imshow("img1", &img1)?;
//     imshow("img2", &img2_bn)?;
//     imshow("img3", &img3)?;
//     wait_key(0)?;
//     destroy_all_windows()?;
//     Ok(())
// }
fn mat_op3() -> Result<()> {
    // 이미지 로드
    let img1 = imread("./img/bike0.png", IMREAD_COLOR)?;
    if img1.empty() {
        println!("Image load failed!");
        return Ok(());
    }

    // 이미지 크기와 영역 정의
    let rect = Rect::new(220, 120, 340, 240);

    // 자른 영역의 이미지 생성
    let mut img2 = Mat::zeros(rect.height, rect.width, img1.typ())?.to_mat()?;
    let mut img3 = img2.clone();

    // 직접 데이터 접근 및 복사
    let src_roi = img1.roi(rect)?.try_clone()?;
    let dst_size = img2.size()?;
    let src_size = src_roi.size()?;
    let src_ptr = src_roi.data_typed::<u8>()?;
    let  dst_ptr = img2.data_typed_mut::<u8>()?;

    for y in 0..src_size.height {
        let src_row = &src_ptr[(y as usize * src_size.width as usize * 3) as usize..];
        let dst_row = &mut dst_ptr[(y as usize* dst_size.width as usize * 3) as usize..];
        dst_row.copy_from_slice(&src_row[..src_size.width as usize * 3]);
    }

    // 반전
    let mut img2_bn = Mat::default();
    bitwise_not(&img2, &mut img2_bn, &Mat::default())?;

    // 결과 출력
    imshow("img1", &img1)?;
    imshow("img2", &img2)?;
    imshow("img3", &img3)?;

    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

pub fn main() -> Result<()> {
    // mat_op1()?;
    // mat_op2()?;
    mat_op3()?;

    Ok(())
}
