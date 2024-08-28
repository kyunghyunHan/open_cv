/*
drawing
Opencv 에서는 화면에 그림을 그릴수 있습니다.


line
Line함수는 영상위에 직선을 그립니다.img영상 위에 pt1좌표부터 pt2좌표까지 직선을 그립니다.선 색상 또는 밝기는 color인자로 지정하며 thinkness인자는 선두꼐를 의미하고 lineType인자는 직선을 그리는 방식을 지정할수 잇습니다.


 img:입출력영상
    pt1:시작점
    pt2:끝점
    colort:선색상
    thickness:선두께
    lineType:선타입
    shift:그리기 좌표값의 축소 비율

    Line Type
    FILLED:내부를 채움
    LINE_4:4방향연결

    LINE_8:8방향 연결
    LINE_AA:안티에얼리싱


    arrowedLine:화살표방향 형태의 직선 (tipLength)
    drawMarke():직전그리기 함수를 이용



marker

    Marker Croess:십자가모양
    Marker_TILTED_CROSS:45도 회전된 모양
    MARKER_STAR:*모양
    MARKER_DIMOND:다이아 모양
    MARKER_SQUARE:정사각형
    MARKER_TRIANGLE_UP 위로 뾰족한 삼각형
    MARKER_TRIANGLE_DOWN 아래로 뾰족한 삼각형


rectangle

사각형을 그리는 함수는 rectangle함수입니다.사각형의 대각 위치에 잇는 두 꼭지점 좌표를 이용하거나 Rect클래스타입의 객체를 이용하여 전달할수 있습니다.

원을 그릴떄 사용하는 함수는 circle() 입니다.중심점 좌표와 반지름을 이용하여 원을그랄수 있습니다.또한 타원을 그리고 싶을떄는 ellipse()함수를 사용할수 있습니다.타원을 그리는 것은 조금 복잡합니다.
임의의 다각형을 그리기 위해서는 polyline()함수를 사용하며  다각형의 꼭지점 좌표를 전달해야 합니다.
/*
img:입출력영상
center:타원의 중심
axes:타원의 반지름
angle:타원 회전각도
startAngle:타원 호의 시작 각도
endAngle:타원 호의 끝 각도
color:타원색상
thickness : 타원 외각선 두꼐
lintType:선타입
shift:그리기 좌표 값의 축소 비율
*/

text



*/

use opencv::{
    core::*,
    highgui::{imshow, wait_key},
    imgcodecs,
    imgproc::{
        get_text_size, put_text, FONT_HERSHEY_COMPLEX, FONT_HERSHEY_COMPLEX_SMALL,
        FONT_HERSHEY_DUPLEX, FONT_HERSHEY_PLAIN, FONT_HERSHEY_SCRIPT_COMPLEX,
        FONT_HERSHEY_SCRIPT_SIMPLEX, FONT_HERSHEY_SIMPLEX, FONT_HERSHEY_TRIPLEX, FONT_ITALIC,
        LINE_AA,
    },
};
use opencv::{
    core::{Mat, Point, Point_, Rect_, Scalar, Size_, Vector, CV_8UC3},
    highgui,
    imgproc::{
        self, MARKER_CROSS, MARKER_DIAMOND, MARKER_SQUARE, MARKER_STAR, MARKER_TILTED_CROSS,
        MARKER_TRIANGLE_DOWN, MARKER_TRIANGLE_UP,
    },
    line_descriptor, Result,
};

use crate::image::{imgshow, put_text};

pub fn main() -> Result<()> {
    // line()?;
    // rectangle()?;
    //text()?; 
    text2()?;
    Ok(())
}

pub fn line() -> Result<()> {
    let mut img =
        Mat::new_rows_cols_with_default(400, 400, CV_8UC3, Scalar::from((255.0, 255.0, 255.0)))?;

    imgproc::line(
        &mut img,
        Point::new(50, 50),
        Point::new(200, 50),
        Scalar::from((0.0, 0.0, 255.0)),
        1,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::line(
        &mut img,
        Point::new(50, 100),
        Point::new(200, 100),
        Scalar::from((255.0, 0.0, 255.0)),
        3,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::line(
        &mut img,
        Point::new(50, 150),
        Point::new(200, 150),
        Scalar::from((255.0, 0.0, 0.0)),
        10,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::line(
        &mut img,
        Point::new(250, 50),
        Point::new(350, 100),
        Scalar::from((0., 0., 255.0)),
        1,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::line(
        &mut img,
        Point::new(250, 70),
        Point::new(350, 120),
        Scalar::from((255., 0., 255.0)),
        1,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::line(
        &mut img,
        Point::new(250, 90),
        Point::new(350, 140),
        Scalar::from((255., 0., 0.)),
        1,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::arrowed_line(
        &mut img,
        Point::new(50, 200),
        Point::new(150, 200),
        Scalar::from((0.0, 0.0, 255.0)),
        1,
        imgproc::LINE_AA,
        0,
        0.0,
    )?;
    imgproc::arrowed_line(
        &mut img,
        Point::new(50, 250),
        Point::new(350, 250),
        Scalar::from((255.0, 0.0, 255.0)),
        1,
        imgproc::LINE_AA,
        0,
        0.0,
    )?;
    imgproc::arrowed_line(
        &mut img,
        Point::new(50, 250),
        Point::new(350, 250),
        Scalar::from((255.0, 0.0, 255.0)),
        1,
        imgproc::LINE_8,
        0,
        0.05,
    )?;

    imgproc::draw_marker(
        &mut img,
        Point::new(50, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_CROSS,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(100, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_TILTED_CROSS,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(150, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_STAR,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(200, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_DIAMOND,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(250, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_SQUARE,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(300, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_TRIANGLE_UP,
        10,
        1,
        imgproc::LINE_8,
    )?;
    imgproc::draw_marker(
        &mut img,
        Point::new(350, 350),
        Scalar::from((0.0, 0.0, 255.0)),
        MARKER_TRIANGLE_DOWN,
        10,
        1,
        imgproc::LINE_8,
    )?;
    highgui::imshow("image", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}

pub fn rectangle() -> Result<()> {
    let mut img =
        Mat::new_rows_cols_with_default(400, 400, CV_8UC3, Scalar::from((255.0, 255.0, 255.0)))?;

    imgproc::rectangle(
        &mut img,
        Rect_::new(50, 50, 100, 50),
        Scalar::from((0.0, 0.0, 255.0)),
        2,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::rectangle(
        &mut img,
        Rect_::new(50, 150, 100, 50),
        Scalar::from((0.0, 0.0, 128.0)),
        -1,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::circle(
        &mut img,
        Point_::new(300, 120),
        30,
        Scalar::from((255.0, 255.0, 0.0)),
        -1,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::circle(
        &mut img,
        Point_::new(300, 120),
        60,
        Scalar::from((255.0, 0.0, 0.0)),
        3,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::ellipse(
        &mut img,
        Point_::new(120, 300),
        Size_::new(60, 30),
        20.0,
        0.0,
        270.0,
        Scalar::from((255.0, 255.0, 0.0)),
        -1,
        imgproc::LINE_AA,
        0,
    )?;
    imgproc::ellipse(
        &mut img,
        Point_::new(120, 300),
        Size_::new(100, 50),
        20.0,
        0.0,
        360.0,
        Scalar::from((0.0, 255.0, 0.0)),
        2,
        imgproc::LINE_AA,
        0,
    )?;

    let mut pts: Vector<Point_<i32>> = Vector::new();
    pts.push(Point::new(250, 250));
    pts.push(Point::new(300, 250));
    pts.push(Point::new(300, 300));
    pts.push(Point::new(350, 300));
    pts.push(Point::new(350, 350));
    pts.push(Point::new(250, 350));
    /*polylines:다각형 그리기 */
    imgproc::polylines(
        &mut img,
        &pts,
        true,
        Scalar::from((255, 0, 255)),
        2,
        imgproc::LINE_AA,
        0,
    )?;
    highgui::imshow("img", &img)?;

    highgui::wait_key(0)?;

    highgui::destroy_all_windows()?;
    Ok(())
}

pub fn text() -> Result<()> {
    let mut img =
        Mat::new_rows_cols_with_default(500, 800, CV_8UC3, Scalar::from((255.0, 255.0, 255.0)))?;

    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_SIMPLEX",
        Point::new(20, 50),
        FONT_HERSHEY_SIMPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_PLAIN",
        Point::new(20, 100),
        FONT_HERSHEY_PLAIN,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_DUPLEX",
        Point::new(20, 150),
        FONT_HERSHEY_DUPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_COMPLEX",
        Point::new(20, 200),
        FONT_HERSHEY_COMPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_TRIPLEX",
        Point::new(20, 250),
        FONT_HERSHEY_TRIPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_COMPLEX_SMALL",
        Point::new(20, 300),
        FONT_HERSHEY_COMPLEX_SMALL,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_SCRIPT_SIMPLEX",
        Point::new(20, 350),
        FONT_HERSHEY_SCRIPT_SIMPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_HERSHEY_SCRIPT_COMPLEX",
        Point::new(20, 400),
        FONT_HERSHEY_SCRIPT_COMPLEX,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;
    imgproc::put_text(
        &mut img,
        "FONT_ITALIC",
        Point::new(20, 450),
        FONT_ITALIC,
        1.0,
        Scalar::from((0, 0, 255)),
        1,
        1,
        false,
    )?;

    imshow("text", &img)?;
    wait_key(0)?;

    Ok(())
}

fn text2() -> Result<()> {
    let mut img =
        Mat::new_rows_cols_with_default(500, 800, CV_8UC3, Scalar::from((255.0, 255.0, 255.0)))?;

    let text = "Hello Open CV";
    let font_face = FONT_HERSHEY_TRIPLEX;
    let font_scale = 2.;
    let thickness = 1;

    let size_text = get_text_size(&text, font_face, font_scale, thickness, &mut 0)?;

    let size_img = img.size()?;

    let org = Point::from((
        (size_img.width - size_text.width) / 2,
        (size_img.height + size_text.height) / 2,
    ));
    imgproc::put_text(
        &mut img,
        text,
        org,
        font_face,
        font_scale,
        Scalar::from((255, 0, 0)),
        thickness,
        LINE_AA,
        false,
    )?;

    imgproc::rectangle(
        &mut img,
        Rect::from_points(org, org + Point::from((size_text.width, -size_text.height))),
        Scalar::from((255, 0, 0)),
        thickness,
        1,
        0,
    )?;
    highgui::imshow("img", &img)?;

    highgui::wait_key(0)?;

    highgui::destroy_all_windows()?;
    
    Ok(())
}
