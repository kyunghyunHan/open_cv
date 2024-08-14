/* Mat op */
/*주요 클래스
Point
- 2차원 평면 위에 있는 점의 좌표를 표현하는 클래스
- 2차원 좌표를 나타내는 x와 y라는 변수를 가지고 있음
- dot()->내적을 계산
- ddot()내적을 실수형으로 계산
- cross() 두 점 사이의 외적을 게산
- inside() 점의 좌표가 사각형 r 안에 있으면 true를 반환

Size
- Size는 사각형 영역의 가로와 세로 크기를 나타내는 width와 height멤버 변수를포함
- area() 함수는 사각형 크기에 해당하는 면적(width x height)을 반환합니다.
- empty() 유효하지 않는 크기이면 true를 반환
- width 는 가로크기
- heigh는 사각형 영역의 세로 크기


Rect
Rect는 사각형의 좌측 상단 점의 좌표를 나타내는 x,y변수와 사각형의 가로 및 세로 크기를 나타내는 width,height변수를 가지고 있습니다.
tl() =>사각형의 좌측 상단 점의 좌표 반환
br() => 사각형의 우측 하단 점의 좌표 반환
size() =>사각형의 크기 정보 반환
arer() => 사각형의 면적
empty()=> 유효하지 않는 사각형이면 True
contains()=>인자로 전달된 pt점이 사각형 내부에 잇으면 true반환
*/
use opencv::{
    core::{
        bitwise_not, no_array, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Point_, Range,
        Rect, Rect_, Scalar, Size, Vector, CV_8UC3,
    },
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    prelude::{MatTraitConstManual, MatTraitManual},
    Result,
};
use std::sync::Arc;
use std::sync::Mutex;

fn point_fn() -> Result<()> {
    let mut pt1: Point_<i32> = Point_::default(); //0,0
    pt1.x = 10;
    pt1.y = 10;

    let pt2 = Point_::from((10, 10));
    Ok(())
}
fn size_fn() -> Result<()> {
    let sz1 = Size::from((10, 20));
    let mut sz2 = Size::default();
    sz2.width = 5;
    sz2.height = 10;

    Ok(())
}
fn rect_fn() -> Result<()> {
    let mut rect: Rect_<i32> = Rect_::default();
    rect.x= 10;
    rect.y= 20;
    rect.width= 10;
    rect.height= 20;

    let rc2 = Rect_::from((10,10,10,10));

    Ok(())
}
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
    let mut range_vector = Vector::new();

    // 범위 추가
    range_vector.push(Range::new(0, 10)?);
    range_vector.push(Range::new(10, 20)?);

    // `&Vector<Range>` 참조 얻기
    let range_vector_ref: &Vector<Range> = &range_vector;
    let a = img1.ranges(&range_vector_ref).unwrap().clone_pointee();
    let img2 = Mat::default();

    // 결과 출력
    imshow("img1", &img1)?;
    imshow("img2", &a)?;

    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

pub fn main() -> Result<()> {
    // mat_op1()?;
    // mat_op2()?;
    // mat_op3()?;
    let rc2 = Rect_::from((10,10,10,10));
    println!("{:?}",rc2);

    Ok(())
}
