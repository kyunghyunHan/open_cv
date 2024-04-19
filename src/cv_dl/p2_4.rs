use opencv::{
    core, highgui,
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst, CAP_DSHOW},
    Result,
};

pub fn main() -> Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; //카메라 연결시도
    let opened = videoio::VideoCapture::is_opened(&cap)?;

    if !opened {
        println!("카메라 연결 실패");
        std::process::exit(0);
    }

    loop {
        let mut frame = core::Mat::default();
        let mut ret = core::Mat::default();
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;
        // cap.read(&mut frame)?;

        highgui::imshow("Video", &frame)?;
        let key = highgui::wait_key(1);
        if key? as u8 as char == 'q' {
            break;
        }
    }
    // cap.release(); //카메라와 연결을 끊음
    highgui::destroy_all_windows()?;
    Ok(())
}
