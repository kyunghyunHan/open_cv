/*영상의 밝기 조절
영상의 밝기 조절이란 영상의 전체적인 밝기를 조절하여 좀 더 발거나 어두운  영상을 만드는 작업입니다.
영상의 밝기를 조절하려면 입력 영상의 모든 픽셀에 양수값을 더하면 영상이 밝아지고 반대로 양수값을 뺴면 영상이 어두워 집니다
밑의 그림은 영상의 밝기를 조절하여 조금 어둡거나 밝은 영상을 만든 예시입니다.

위 수식에서 src는 입력영상,dst는 출력영상,n은 조절할 밝기 값을 나타냅니다.n이 양수이면 출력영상dst의 전체적인 밝기가 증가하고 n이 음수이면 밝기가 감소하여 어두워집니다.
하지만 수식을 그대로 적용한다면 결과영상의 픽셀값이 255보다 커지거나 0보다 작아지는 경우가 발생할수 잇습니다.
그러나 255보다 큰값을 결과영상의 픽셀값으로 설정할수 없기 때문에 0 이나 255로 고정되게 됩니다.

다음은 밝기를 +100 하는 예시 코드입니다.
원본 이미지와 밝기를 +100 한 이미지 화면을 출력하게 됩니다.


영상의 밝기 조절 직접 구현하기
만약 opencv가 제공하는 연산자 함수를 사용하지 않고 직접 영상의 밝기를 조절하려면 다음과 같이 Mat행렬의 원소값을 참조하여 밝기를 조절할수 있습니다.

하지만 에러가 나타나게 됩니다 그것은 값이 255을 초과하면 안대는데 255를 초과햇기때문에 overflow가 발생합니다.

다음과 같이 작성하면 overflow를 피하며 값을 저장할수 잇습니다.v i32타입이기때문에 255보다 큰 정수도 저장가능하기 때문에 over flow를 피할수 있습니다.


만약 증가한 영상이 마음에 들지않는다면 트랙바를 통해 밝기를 조절할수 있습니다.
*/
use opencv::{
    core::{
        self, add, add_mat_scalar, subtract, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Scalar, Vec3b
    },
    highgui::{self, create_trackbar, destroy_all_windows, imshow, named_window, wait_key},
    imgcodecs::{self, IMREAD_COLOR, IMREAD_GRAYSCALE},
    Result,
};
use std::sync::{Arc, Mutex};
pub fn main() -> Result<()> {
    // brihtness()?;
    // brihtness2()?;
    brihtness3()?;

    Ok(())
}

fn brihtness() -> Result<()> {
    // Load the source image
    let src = imgcodecs::imread("./img/bike0.png", IMREAD_COLOR)?;

    if src.empty() {
        panic!("{}", "error");
    }

    let dst = add_mat_scalar(&src, Scalar::all(100.))?.to_mat()?;
    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn brihtness2() -> Result<()> {
    let src = imgcodecs::imread("./img/lenna.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("{}", "image load fiiled");
    }
    let mut dst = src.clone();
    // for j in 0..src.rows() {
    //     for i in 0..src.cols() {
    //         let v = *src.at_2d::<u8>(j, i)? as i32 + 100;
    //         *dst.at_2d_mut::<u8>(j, i)? = *src.at_2d::<u8>(j, i)?  + 100;
    //     }
    // }
    for j in 0..src.rows() {
        for i in 0..src.cols() {
            let v = *src.at_2d::<u8>(j, i)? as i32 + 100;
            *dst.at_2d_mut::<u8>(j, i)? = if v > 255 {
                255
            } else if v < 0 {
                0
            } else {
                v as u8
            }
        }
    }

    highgui::imshow("Original Image", &src)?;
    highgui::imshow("Brightened Image", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn brihtness3() -> Result<()> {
    let img = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/bike0.png",
        IMREAD_COLOR,
    )?));
    let img_clone = Arc::clone(&img);

    let window = "face detection";
    named_window(window, highgui::WINDOW_AUTOSIZE)?;
    create_trackbar(
        "face detection",
        "face detection",
        None,
        100,
        Some(Box::new({
            move |val| {
                let img_guard = img_clone.lock().unwrap();
                let brightness_value = val as f64;
                let dst = add_mat_scalar(&*img_guard, Scalar::all(brightness_value)).unwrap().to_mat().unwrap();
                imshow(window, &dst).unwrap();
            }
        })),
    )?;

    imshow(window, &*img.lock().unwrap())?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
