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

rc1에 size(50 ,40) 을 더하면 rc1의 가로 크기와 세로 크기가 각각 50과 40만큼 증가
rc4는 rc2에서 Point (10,10 )만큼 증가
&연산은 두 사각형이 교차하는 사각형 영역
| 연산은 두 사격형을 모두 포함하는 최소 크기의 사각형을 반환

RotatedRect클래스
회전된 사각형을 포현하는 클래스
회전된 사각형의 중심 좌표를 나타내는 center,
사각형의 가로 및 세로 크기를 나태내는 size
회전 각도 정보를 표현하는 angle을 멤버 변수로 가짐

-points는 회전된 사각형의 네 꼭지점 좌표를 pts에 인자에 담아 반환
- bounddingRect()함수는 회전된 사각형의을 포함하는 최소 크기의 사각형 정보를 반환(정수 단위)
- bounddingRect2f()함수는 회전된 사각형의을 포함하는 최소 크기의 사각형 정보를 반환(실수단위)

Range

범위 또는 구간을 표현하는 클래스
시작과 끝을 나타내는 start와 end멤벼 변수를 가지고 있음
size() = 범위크기 (end -start)반환
empty() start와 end가 같으면 true반환
all() start=INN_MIN ,end =INT_MAX로 설정한 Range객체 반환



Mat

일반적인 2차원 행렬 뿐만 아니라 고차원 행렬까지 표현
한개이상의 채널을 가질수 있음
정수,실수,복소수,등으로 구성된 행렬 또는 Vector을 저장 가능
컬러영상 또는 크레이스케일 영상 저장 가능
경우에 따라서 Vector Field ,Poind cloud,Tensor,histogram등을 저장하는 용도로 사용
-dims Mat행렬의 차원을 나타냄
- rows,cols 2차원 행렬의 크기를 나타냄 3차원 이상에서부터는 -1이 저장,3차원 이상부터는 size멤버 변수를 이용하여 참조 가능
- data 는 행렬의 원소데이터가 저장되어 있는 메모리 공간을 가리키는 포인터형 멤버 변수 만약 하우것도 저장도있지 않으면 None값을 가짐
- CV_8U   0
- CV_8S   1
- CV_16U  2
- CV_16S  3
- CV_32S  4
- CV_32F  5
- CV_64F  6
- CV_16F  7

- Mat행렬 원소는 하나의 값을 가질수도 있고 여러개의 구성된 값을 가질수도 있음,Mat행렬원소를 구성하는 각각의 값을 채널 이라고 부름
- 하나의 행렬을 구성하는 각 채널은 모두 같은 자료형 사용해야함 예 그레이스케일 영상은 하나의 픽셀이 밝기 정보 하나만 사용하므로 1채널 행렬로 표현
- 컬러 영상의 경우 하나의 픽셀이 BGR세개의 색상정보를 가지고 있으므로 3채널 행렬로 표현
- Mat행렬의 깊이 정보와 채널 수 정보를 합쳐서 Mat객체의 type이라 함
- 행렬 뒤에 깊이 표현 매크도 뒤에 C1,C3같은 채널 정보가 추가로 붙어진 형태 즉 CV_8UC1타입은 8비트 u8자료형을 사용하고 채널이 한개인 행렬또는 영상을 의미
- BGR세개의 생삭 성분을 가지고 있는 컬러 영상은 u8자료형 및 세개의 채널을 가지고 있기 떄문에 CV_8UC3 타입입니다.
- 복소수처럼 두개의 실수 값을 사용하고 있는 행렬은  CV_32FC2타입으로 만들수 있음
*/
use opencv::{
    core::{
        bitwise_not, no_array, Mat, MatExprTraitConst, MatTrait, MatTraitConst, Point2f, Point_,
        Range, Rect, Rect_, RotatedRect, Scalar, Size, Size2f, Size_, Vector, CV_32FC1, CV_32SC1,
        CV_8UC1, CV_8UC3,
    },
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    prelude::{MatTraitConstManual, MatTraitManual},
    Result,
};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::Mutex;
fn point_fn() -> Result<()> {
    let mut pt1: Point_<i32> = Point_::default(); //0,0
    pt1.x = 10;
    pt1.y = 10;

    let pt2 = Point_::from((10, 10));
    println!("{:?}", pt1);
    println!("{:?}", pt2);
    Ok(())
}
fn size_fn() -> Result<()> {
    let sz1 = Size::from((10, 20));
    let mut sz2 = Size::default();
    sz2.width = 5;
    sz2.height = 10;
    println!("{:?}", sz1);
    println!("{:?}", sz2);
    Ok(())
}

fn rect_fn() -> Result<()> {
    let mut rc1: Rect_<i32> = Rect_::default(); //{ x: 0, y: 0, width: 0, height: 0 }

    //[0 * 0 from (0,0)]
    let rc2 = Rect_::from((10, 10, 60, 40));
    //{ x: 10, y: 10, width: 60, height: 40 }

    let rc3 = rc1 + Size::from((50, 40)); //{ x: 0, y: 0, width: 50, height: 40 }

    let rc4 = rc2 + Point_::from((10, 10)); //{ x: 20, y: 20, width: 60, height: 40 }
    let rc5 = rc3 & rc4; //{ x: 20, y: 20, width: 30, height: 20 }
    let rc6 = rc3 | rc4; //{ x: 0, y: 0, width: 80, height: 60 }

    println!("{:?}", rc1);
    println!("{:?}", rc2);
    println!("{:?}", rc3);
    println!("{:?}", rc4);
    println!("{:?}", rc5);
    println!("{:?}", rc6);

    Ok(())
}

fn rotated_rect_fn() -> Result<()> {
    //중심 좌표가 (40,30) 크기는 40x20 시게 방향으로 30 % 만큼 회전된 사각형
    let rr1: RotatedRect =
        RotatedRect::new(Point2f::from((40., 30.)), Size2f::from((40., 20.)), 30.)?;
    println!("{:?}", rr1);
    //배열에 제 꼭지점 좌표가 pts배열에 저장
    let mut pts: [Point2f; 4] = [Default::default(); 4];
    rr1.points(&mut pts)?;
    println!("{:?}", pts);
    //경우에 따라서 회전된 사각형을 감사는 최소 크기의 사각형 정보가 필요
    //특정 개체를 감싸는 최소 크기의 사각형을 bounding box함
    let br = rr1.bounding_rect()?;
    //[0 * 0 from (0,0)]
    // let rr2 = Rect_::from((10, 10, 60, 40));
    // //{ x: 10, y: 10, width: 60, height: 40 }

    // let rr3 = rr1 +Size::from((50,40));//{ x: 0, y: 0, width: 50, height: 40 }

    // let rr4 =rr2+Point_::from((10,10));//{ x: 20, y: 20, width: 60, height: 40 }
    // let rr5 = rr3 & rr4;     //{ x: 20, y: 20, width: 30, height: 20 }
    // let rr6  =rr3 | rr4;    //{ x: 0, y: 0, width: 80, height: 60 }

    // println!("{:?}", rr2);
    // println!("{:?}", rr3);
    // println!("{:?}", rr4);
    // println!("{:?}", rr5);
    // println!("{:?}", rr6);

    Ok(())
}

fn mat_fn() -> Result<()> {
    //단순히 Mat타입 선언
    let img1: Mat = Mat::default();
    //Mat클래스에 영상의 크기를 지정할떄 가로세로 순이 아닌 세로 가로 순
    let img2: Mat = unsafe { Mat::new_nd(&[480, 640], CV_8UC1)? }; //{ width: 640, height: 480 }
    let img3: Mat = unsafe { Mat::new_nd(&[480, 640], CV_8UC3)? };
    //Size는가로세로 크기순으로 크기를 지정하
    //그런데 이처럼 행렬의 크기와 타입을 지정하여 Mat객체를 생성할 경우 행렬의 모든 원소는 garbage value라는 임의의 값으로 채워지게 됨
    //그러므로 Mat객체를 생성함과 동시에 모든 원소값을 특정 값으로 초기화 하여 사용하는 것이 안전
    //Scalar는 네개의 실수 값을 저장 가능한 클래스이며 주료 영상의 픽셀 값을 표현하는 용도로 사용
    let img4 = unsafe { Mat::new_size(Size_::from((640, 480)), CV_8UC3)? };

    let img5 = Mat::new_size_with_default(Size::new(640, 480), CV_8UC1, Scalar::all(128.0))?; //모든 픽셀값이 128로 지정된 그레이스케일 영 상
    let img6 = Mat::new_size_with_default(Size::new(640, 480), CV_8UC3, Scalar::from((0, 0, 255)))?; //모든 픽셀값이 빨간색으로 지정된 컬러 영상
                                                                                                     //Scalar 클래스를 이용하여  컬러 영상의 색상을 지정할떄는 BGR색상 순으로 값을 지정
                                                                                                     //새로운 행렬을 생성할떄 모든 원소값을 0으로 초기화 하는 경우가 많아 Mat클래스에 Scalar을 0으로 지정, 이러한 용도로 Mat::zero()함수 사용
                                                                                                     //MatExpr은 행렬의 대수연산을 표현하는 클래스
    let mat1 = Mat::zeros(3, 3, CV_32SC1)?.to_mat()?; //zerors()를 사용할 경우 MatExpr클래스로 반환되기 떄문에 to Mat()함수를 사용해 Mat으로 변환해주어야함
                                                      //모든 원소기 1로 초기화된 행렬을 생성하려면 Mat::ones()
                                                      //또한 행렬 연산에서 자주 사용되는 단위행렬을 생성하려면 Mat::eye()를 사용

    let mat2 = Mat::ones(3, 3, CV_32FC1)?.to_mat()?;
    let mat3 = Mat::eye(3, 3, CV_32FC1)?.to_mat()?;

    //Mat객체를 생성할떄 행렬원소를 저장할 메모리 공간을 새로 할당하는것이 아니라 기존에 이미 할당되어 있는 메모리 공간의 데이터를 행렬 원소 값으로 사용가능
    //외부 메모리 공간을 활용하여 Mat객체를 생성한다는 것은 자체적인 메모리 할당을 수행하지 않고 외부 메모리를 참조하는 것이기 떄문에 객체 생성이 빠른 장점이 있음

    // float data[]={1,2,3,4,5,6}
    // Mat mat4(2,3,CV_32FC1)
    let data = [1., 2., 3., 4., 5., 6.];
    //이 방법은 Boxed<Mat>을반환 완전한 Mat으로 가져오려면 복제를 해야함
    //clone_pointee은 Mat내부 데이터를 복제하여 새로운 Mat객체를 생성하며 이 메서드는 원본 데이터와 완전히 독립적인 복사본을 만듭니다.
    //try_clone은 기본적으로 Mat객체의 헤더를 복제하고  내부 데이터는 참조 카운팅을 통해 공휴하기때문에 두 객체는 동일한 데이터를 가리키며 기존 데이터가 바뀌면 같이 바뀌기 때문에 데이터 복사를 피하고 메모리 사용을 줄이고 싶을떄 사용합니다.
    //외부 배열을 행렬 원소 값으로 사용하고자 할 경우 외부 배열 크기와 생성할 행렬 원소 개수는 같아야함,서로 사용하는 자료형도 같아야함
    // let mat4 =Mat::new_rows_cols_with_data(2, 3, &data)?;
    // let mat4 =Mat::new_rows_cols_with_data(2, 3, &data)?.clone_pointee();
    let mut mat4 = Mat::new_rows_cols_with_data(2, 3, &data)?.try_clone()?;

    let mut mat5 = unsafe { Mat::new_rows_cols(2, 3, CV_32FC1)? };
    let data: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    for (i, &value) in data.iter().enumerate() {
        *mat5.at_mut::<f32>(i as i32)? = value;
    }
    let data: [[f32; 6]; 6] = [
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    ];
    let data: Vec<Vec<f32>> = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    ];
    let mat5 = Mat::from_slice_2d(&data)?;
    //이미 데이터가 할당 되어 있는 객체에 create함수를 호출할 경우 create함수의 인자로 지정한 행렬크기와 타입이 기존 행렬과 같으면 별다른 동작을 하지 않지만 새로 만들 행렬의 크기또는 타입이 기존 행렬과 다를 경우 기존 메모리 공간을 해제한 후 새로운 행렬 데이터 저장을 위한 메모리 공간을 할당한 할당
    //create함수는 초기화 하는 기능은 없기때문에 핼렬 전체의 원소 값을 초기화 하고싶다면 set_to 함수를 사용해야함
    unsafe { mat4.create_size(Size::from((256, 256)), CV_8UC3)? };

    println!("{:?}", mat4);
    println!("{:?}", img4);

    Ok(())
}

fn mat_op1() -> Result<()> {
    let img1 = Mat::default();
    let img2 = unsafe { Mat::new_rows_cols(480, 640, CV_8UC1)? };
    let img3 = unsafe { Mat::new_rows_cols(480, 640, CV_8UC3)? };
    let img4 = unsafe { Mat::new_size(Size_::from((640, 480)), CV_8UC3)? };
    let img5 = Mat::new_size_with_default(Size::new(640, 480), CV_8UC1, Scalar::all(128.0))?; //모든 픽셀값이 128로 지정된 그레이스케일 영 상
    let img6 = Mat::new_size_with_default(Size::new(640, 480), CV_8UC3, Scalar::from((0, 0, 255)))?; //모든 픽셀값이 빨간색으로 지정된 컬러 영상

    let mat1 = Mat::zeros(3, 3, CV_32SC1)?.to_mat()?;
    let mat2 = Mat::ones(3, 3, CV_32FC1)?.to_mat()?;
    let mat3 = Mat::eye(3, 3, CV_32FC1)?.to_mat()?;

    let data = [1., 2., 3., 4., 5., 6.];
    let mut mat4 = Mat::new_rows_cols_with_data(2, 3, &data)?.try_clone()?;
    let mut mat5 = unsafe { Mat::new_rows_cols(2, 3, CV_32FC1)? };
    let data: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    for (i, &value) in data.iter().enumerate() {
        *mat5.at_mut::<f32>(i as i32)? = value;
    }

    unsafe { mat4.create_size(Size::from((256, 256)), CV_8UC3)? };
    unsafe { mat5.create_size(Size::from((4, 4)), CV_32FC1)? };
    mat4.set_scalar(Scalar::from((255, 0, 0)))?;//모든 픽셀을 파란색으로 설정
    mat5.set_to(&1., &no_array())?;//mat5의 모든 원소 값은 1.로 설정
    imshow("img1", &img1)?;
    imshow("img2", &img2)?;
    imshow("img3", &img3)?;
    wait_key(0)?;

    Ok(())
}

fn mat_op2() -> Result<()> {
    //행렬복사
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
fn mat_op3() -> Result<()> {
    let img1 = imread("./img/bike0.png", IMREAD_COLOR)?;
    if img1.empty() {
        panic!("image load failed");
    }
    
    // let rect = Rect::new(220, 120, 340, 240);
    // let img2 = Mat::roi(&img1, rect)?;
    // let img3 = Mat::roi(&img1, rect)?.try_clone()?;
    // let mut img2_bn = Mat::default();
    // // img2를 반전
    // bitwise_not(&img2, &mut img2_bn, &Mat::default())?;

    // imshow("img1", &img1)?;
    // imshow("img2", &img2_bn)?;
    // imshow("img3", &img3)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
// fn mat_op3() -> Result<()> {
//     // 이미지 로드
//     let img1 = imread("./img/bike0.png", IMREAD_COLOR)?;
//     if img1.empty() {
//         println!("Image load failed!");
//         return Ok(());
//     }
//     let mut range_vector = Vector::new();

//     // 범위 추가
//     range_vector.push(Range::new(0, 10)?);
//     range_vector.push(Range::new(10, 20)?);

//     // `&Vector<Range>` 참조 얻기
//     let range_vector_ref: &Vector<Range> = &range_vector;
//     let a = img1.ranges(&range_vector_ref).unwrap().clone_pointee();
//     let img2 = Mat::default();

//     // 결과 출력
//     imshow("img1", &img1)?;
//     imshow("img2", &a)?;

//     wait_key(0)?;
//     destroy_all_windows()?;

//     Ok(())
// }

pub fn main() -> Result<()> {
    // mat_op1()?;
    // mat_op2()?;
    // mat_op3()?;
    // rect_fn()?;
    // rotated_rect_fn()?;
    mat_fn()?;
    Ok(())
}
