/*
drawing

line

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

rectangle


text

*/


use opencv::{
    core::{Mat, Point, Scalar, CV_8UC3,Rect_,Point_,Vector,Size_},
    highgui,
    imgproc::{self, MARKER_CROSS, MARKER_DIAMOND, MARKER_SQUARE, MARKER_STAR, MARKER_TILTED_CROSS, MARKER_TRIANGLE_DOWN, MARKER_TRIANGLE_UP},
    line_descriptor, Result,
    
};
use opencv::{core::*, highgui::{imshow, wait_key}, imgcodecs, imgproc::{ FONT_HERSHEY_SIMPLEX,FONT_HERSHEY_PLAIN,FONT_HERSHEY_DUPLEX,FONT_HERSHEY_COMPLEX,FONT_HERSHEY_TRIPLEX,FONT_HERSHEY_COMPLEX_SMALL,FONT_HERSHEY_SCRIPT_SIMPLEX,FONT_HERSHEY_SCRIPT_COMPLEX,FONT_ITALIC}};

pub fn main()->Result<()> {
    // line()?;
    // rectangle()?;
    text()?;
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
    imgproc::polylines(&mut img, &pts, true, Scalar::from((255,0,255)), 2, imgproc::LINE_AA, 0)?;
    highgui::imshow("img", &img)?;

    highgui::wait_key(0)?;

    highgui::destroy_all_windows()?;
    Ok(())
}



pub fn text()->Result<()>{
    let mut img =
    Mat::new_rows_cols_with_default(500, 800, CV_8UC3, Scalar::from((255.0, 255.0, 255.0)))?;
   
   imgproc::put_text(&mut img,"FONT_HERSHEY_SIMPLEX",Point::new(20, 50),FONT_HERSHEY_SIMPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_PLAIN",Point::new(20, 100),FONT_HERSHEY_PLAIN,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_DUPLEX",Point::new(20, 150),FONT_HERSHEY_DUPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_COMPLEX",Point::new(20, 200),FONT_HERSHEY_COMPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_TRIPLEX",Point::new(20, 250),FONT_HERSHEY_TRIPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_COMPLEX_SMALL",Point::new(20, 300),FONT_HERSHEY_COMPLEX_SMALL,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_SCRIPT_SIMPLEX",Point::new(20, 350),FONT_HERSHEY_SCRIPT_SIMPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_HERSHEY_SCRIPT_COMPLEX",Point::new(20, 400),FONT_HERSHEY_SCRIPT_COMPLEX,1.0,Scalar::from((0,0,255)),1,1,false)?;
   imgproc::put_text(&mut img,"FONT_ITALIC",Point::new(20, 450),FONT_ITALIC,1.0,Scalar::from((0,0,255)),1,1,false)?;

    imshow("text",&img)?;
    wait_key(0)?;
    
    Ok(())
}
