use opencv::{
    core::{self, MatTraitConst, Point, Scalar},
    highgui::{self, imshow, wait_key},
    imgcodecs,
    imgproc,
    Result
};
/*
color:색상
org:텍스트를 쓸 위치
font_face = 글꼴
font_scale = 크기
tcickness = 글씨 두꼐
line_type =원점
*/
pub fn main() -> Result<()> {
    let mut source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    println!("{:?}",source_img.size()?.width);
    let color = Scalar::new(255.0, 0.0, 0.0, 0.0); // Example color, replace with actual color
    let org = Point::new(source_img.size()?.width/2, source_img.size()?.height/2); // Example origin, replace with actual position
    let font_face = imgproc::FONT_HERSHEY_SIMPLEX;
    let font_scale = 1.0;
    let thickness = 2;
    let line_type = 8;
    let bottom_left_origin = false;

    imgproc::put_text(
        &mut source_img, 
        "aa", 
        org, 
        font_face, 
        font_scale, 
        color, 
        thickness, 
        line_type, 
        bottom_left_origin,
    )?;

    highgui::imshow("hello opencv!", &source_img)?;
    wait_key(10000)?;

    Ok(())
}
