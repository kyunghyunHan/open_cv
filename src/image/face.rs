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
    let mut image = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;

    // 그레이스케일로 변환
    let mut gray = core::Mat::default();
    imgproc::cvt_color_def(&image, &mut gray, imgproc::COLOR_BGR2GRAY)?;

    // 얼굴 감지
    let mut faces = types::VectorOfRect::new();
    face_detector.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        2,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size { width: 30, height: 30 },
        core::Size { width: 0, height: 0 },
    )?;

    // 감지된 얼굴에 사각형 그리기
    for face in faces.iter() {
        let scaled_face = core::Rect::new(
            face.x * 4,
            face.y * 4,
            face.width * 4,
            face.height * 4,
        );
        imgproc::rectangle(&mut image, scaled_face, core::Scalar::new(0.0, 255.0, 0.0, 0.0), 2, 8, 0)?;
    }

    // 결과를 화면에 표시
    highgui::imshow(window, &image)?;
    highgui::wait_key(0)?;

    Ok(())
}
