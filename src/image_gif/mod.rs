use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    Result,
};
use image::AnimationDecoder;
use image::codecs::gif::GifDecoder;
use image::{DynamicImage, GenericImageView};
use std::fs::File;

fn load_gif_frame_as_opencv_mat(gif_path: &str) -> Result<Mat> {
    // GIF 파일을 열고 디코더로 읽기
    let file = File::open(gif_path).unwrap();
    let mut decoder = GifDecoder::new(file).unwrap();

    // 첫 번째 프레임을 가져오기
    if let Some(frame) = decoder.into_frames().next() {
        let frame = frame.unwrap().into_buffer();
        let dynamic_image = DynamicImage::ImageRgba8(frame);

        // BGR로 변환 후 OpenCV Mat로 변환
        let frame_bgr = dynamic_image.to_rgb8();
        let (width, height) = dynamic_image.dimensions();

        // 데이터를 복사하여 새로운 Mat 생성
        let mat_data = frame_bgr.as_raw();
        let mat_size = (height as i32, width as i32);
        let mut mat = Mat::new_rows_cols_with_default(mat_size.0, mat_size.1, core::CV_8UC3, core::Scalar::default())?;
        
        // mat에 데이터를 복사합니다.
        mat.data_typed_mut()?.copy_from_slice(mat_data);
        
        return Ok(mat);
    }

    Err(opencv::Error::new(0, "Failed to load GIF frame".to_string()))
}

pub fn main() -> Result<()> {
    let window = "GIF Image";

    // 윈도우 생성
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    // GIF 이미지 불러오기
    let gif_path = "./img/cat_gif.gif"; // 이 경로를 자신의 GIF 파일 경로로 변경하세요
    let image = load_gif_frame_as_opencv_mat(gif_path)?;

    // 결과를 화면에 표시
    highgui::imshow(window, &image)?;
    highgui::wait_key(0)?;

    Ok(())
}
