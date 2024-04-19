use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    types,
    Result,
    imgcodecs,
};
use opencv::prelude::CascadeClassifierTrait;

pub fn main() -> Result<()> {
    let window = "face detection";

    // 윈도우 생성
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    // 얼굴 감지기 초기화
    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let mut face_detector = objdetect::CascadeClassifier::new(&xml)?;

    // 이미지 파일에서 이미지 읽기
    let mut image = imgcodecs::imread("./img/가족2.jpeg", imgcodecs::IMREAD_COLOR)?;

    // 그레이스케일로 변환
    let mut gray = core::Mat::default();
    imgproc::cvt_color_def(&image, &mut gray, imgproc::COLOR_BGR2GRAY)?;

    // 얼굴 감지
    let mut faces = types::VectorOfRect::new();
    face_detector.detect_multi_scale(
        &gray,
        &mut faces,
        1.3, // 이미지 파라미드 스케일
        1,   // 인접객체 최소거리
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size { width: 10, height: 10 }, // 탐지 객체
        core::Size { width: 1000, height: 1000 },
    )?;

    // 감지된 얼굴에 사각형 그리기
    for face in faces.iter() {
        let scaled_face = core::Rect::new(
            face.x * 1,
            face.y * 1,
            face.width * 1,
            face.height * 1,
        );
        imgproc::rectangle(&mut image, scaled_face, core::Scalar::new(255.0, 0.0, 0.0, 0.0), 2, 8, 0)?;

        // 얼굴 위에 텍스트 추가
        let text = format!("Face: ({}, {})", face.x, face.y);
        let org = core::Point::new(face.x, face.y - 10);
        imgproc::put_text(&mut image, &text, org, imgproc::FONT_HERSHEY_SIMPLEX, 0.5, core::Scalar::new(0.0, 255.0, 0.0, 0.0), 1, imgproc::LINE_AA, false)?;
    }

    println!("Detected faces: {}", faces.len());

    // 결과를 화면에 표시
    highgui::imshow(window, &image)?;
    highgui::wait_key(0)?;

    Ok(())
}
