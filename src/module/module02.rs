use opencv::core::{MatExprTraitConst, MatTrait, Scalar, Size, Vector, CV_8UC1, CV_8UC3, DECOMP_LU};
use opencv::{
    core::{
        add,
        add_def,
        self, MatTraitConst, Mat_, Mat_AUTO_STEP, Point2f, Point_, Rect_, RotatedRect, Size2f,
        Size_, CV_32FC1, CV_32SC1,
    },
    highgui,
    imgcodecs::{self, IMWRITE_JPEG_QUALITY},
    types::VectorOfi32,
    Result,
    imgproc::resize,
    imgproc
};
pub fn main() -> Result<()> {
    /*Point:픽셀의 좌표를 구함
    dot(내적을 구함)
    ddot(내적을 실수형으로 계산함)
    cross(외적을구함)
    inside(점의좌표가 사각형 r영역 안에 있으면 true를 반환)
    */

    let pt1 = Point_::new(5, 10);
    let pt2 = Point_::new(10, 30);

    let pt3 = pt1 + pt2;
    println!("{:?}", pt3);
    let pt4 = pt1.dot(pt2);
    println!("{:?}", pt4);

    /*SIZE:사각형 영역의 크기를 구함
    area()사각형의 크기에 해당하는 면적(width * height)을반환
    empty:유효하지 않는 크기이면 true를 반환
    width:가로크기,height:세로크기


     */
    let mut sz1 = Size_::new(0, 0);
    sz1.width = 5;
    sz1.height = 10;
    let sz2 = Size_::new(10, 20);
    let sz3 = sz1 + sz2;
    let sz4 = sz1 * 2;
    let area1 = sz4.area();
    println!("{}", area1);

    /* RECT:사각형의 위치와 크기정보를 나타냄
     tl():사각형의 좌측 상단 점의 좌표를 반환
     br():사각형의 우측 하단 점의 좌표를반환
     size():멤버 함수는 사각형의 크기 정보를 번환
     area():사각형의 면적을 반환
     empty:유효하지 않는 사각형이면 true를반환
     contains:인자로 전달된 pt점이 사각형 내부에 있으면 ture를 반환



    */
    let rc1: Rect_<i32> = Rect_::default();
    let rc2 = Rect_::new(10, 10, 60, 40);
    let rc3 = rc1 + Size_::new(50, 40);
    let rc4 = rc2 + Point_::new(10, 10);
    let rc5 = rc3 & rc4;
    let rc6 = rc3 | rc4;

    /*
    RotatedRect:회전된 사각형을 표현
    points:멤버 함수는 회전된 사각형은 네꼭지점 좌표를 pts인자에저장
    boundingRect():회전된 사각형을 포함하는 최소 크기의 사각형 정보를 저장
    boundingRect2f():멤버 함수는 회전된 사각형을 포함하는 최소 크기의 사각형 정보르 ㄹ반환
    */

    let rr1 = RotatedRect::new(Point2f::new(40.0, 30.0), Size2f::new(40.0, 20.0), 30f32)?;
    let mut pts: [Point_<f32>; 4] = Default::default();
    rr1.points(&mut pts)?;
    println!("{:?}", pts);

    /*Range
    - 범위 또는 구간을 표현하는 클래스
    size = end-start를 반환
    empty= start와 end가 같으면 true를반환
    all= start= INT_MIN,end= INT_MAX로 설정한 Range객체를반환
     */

    /*Mat
     - 정수,실수,복소수 등으로 구성된 행렬 또는 벡터를 저장가능
    dims= 차원
    rows,cols = 2차원 행렬의크기
    3차원이상 =>size이용 참조


    ones - 1로 초기화된 행렬
    colRange:
    row
    col

     */
    let img2 = core::Mat::new_rows_cols_with_default(480, 640, CV_8UC1, core::Scalar::all(0.0))?;
    let img3 = core::Mat::new_rows_cols_with_default(480, 640, CV_8UC3, core::Scalar::all(0.0))?;

    println!("{:?}", img2);
    println!("{:?}", img3);

    let img5 = core::Mat::new_rows_cols_with_default(480, 640, CV_8UC1, core::Scalar::from(128.0))?;
    let img6 = core::Mat::new_rows_cols_with_default(
        480,
        640,
        CV_8UC3,
        core::Scalar::from((0.0, 0.0, 255.0)),
    )?;
    println!("{:?}", img5);
    println!("{:?}", img6);
    /* CV_8UC1 이면 uchar  CV_32_FC1:flaot CV_8UC3 사용 Vec3b자료형*/
    let mat1 = core::Mat::zeros(3, 4, CV_32SC1)?.to_mat()?;

    let test = mat1.at::<i32>(0)?;
    let test = mat1.at_2d::<i32>(1, 1)?;
    for j in 0..mat1.rows(){
        for i in 0..mat1.cols(){
            println!("{}",mat1.at_2d::<i32>(1, 1)?);
        }
    }
    println!("{}", test);

    /*ptr :특정 행의 첫번쨰 원소 주소르 반환*/

    for j in 0..mat1.rows(){
        println!("{:?}",mat1.ptr(j)?);
        for i in 0..mat1.cols(){
         
        }
    }
    mat_op4()?;
    // mat_op6()?;
    /*Matiterator_ */
    Ok(())
}

fn mat_op4() -> opencv::Result<()> {
    let mut mat1 = core::Mat::zeros(3, 4, CV_32SC1)?.to_mat()?;
    for j in 0..mat1.rows() {
        for i in 0..mat1.cols() {
            // Accessing and modifying the element at (j, i)
            *mat1.at_2d_mut::<i32>(j, i)?+=1;
            println!("{:?}",mat1.at_2d::<i32>(j, i)?      )
        }
    }
    for i in 0..mat1.rows() {
        let  p = mat1.ptr_mut(i)?;
        for j in 0..mat1.cols() {
            unsafe {
                *p.offset(j as isize) += 1;
                println!("{:?}",*p.offset(j as isize));
            }
        }
    }
   
   println!("{}",mat1.channels());//행렬의 채널의 수
   println!("{}",mat1.depth());//행렬의 깊이
   println!("{:?}",mat1.elem_size()?);//한개의 원소가 차지하는 메모리 크기를 바이트 단위로 반환
   println!("{}",mat1.empty());//비어있는지 아닌지 확인
   println!("{}",mat1.is_continuous());//각 행의 원소가 연속적으로 저장되어 있으면 true
   println!("{}",mat1.is_submatrix());//행렬이 다른 행렬의 부분 행렬이면 true
   println!("{:?}",mat1.size());//행렬크기를 size타입으로 반환
   println!("{}",mat1.total());//전체 원소의 개수
   println!("{:?}",mat1.typ());//타입 반환



    Ok(())
}
   /*Matrix Operations */
fn mat_op6() -> opencv::Result<()> {
    let data = vec![1, 1, 2, 3];
    /* Create a 2x2 matrix with the specified data */
    let mut mat1 = core::Mat::new_rows_cols_with_default(2, 2, CV_32FC1, core::Scalar::new(1.0, 0.0, 0.0, 0.0))?;
    /*inverse Matrix */
    let mat2 = mat1.inv(DECOMP_LU).unwrap().to_mat()?;
    let mut sum_mat =core::Mat::default();
    let add = add_def(&mat1, &mat2,&mut sum_mat)?;
    /*transpose martrix :전치행렬 */
    println!("transpose martrix:{:?}",mat1.t()?);
    println!("add martrix:{:?}",add);
    /*reshape:행렬의 모양을 변경*/
    let mat3= mat2.reshape(0, 1)?;
    let mut dst= core::Mat::default();
    resize(&mat3, &mut dst, Size::from((1,1)), 0.0, 0.0,  imgproc::INTER_LINEAR)?;
    println!("resize martrix:{:?}",dst);
    let mat4= core::Mat::ones(2, 2, CV_32FC1)?.to_mat()?;
    mat1.push_back(&mat4)?;
    Ok(())
}