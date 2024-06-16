use opencv::videoio::{VideoCapture, CAP_ANY, CAP_DSHOW, CAP_PROP_FPS};
use opencv::{
    core::{no_array, Size_},
    highgui::{self, destroy_all_windows},
    imgproc,
    prelude::*,
    videoio::{self, CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH},
    Result,
};
use opencv::core::hconcat;
use crate::image::imgshow;
/*Video Capture */
pub fn main() -> Result<()> {
    // video_capture()?;
    video_add_capture()?;
    Ok(())
}
pub fn video_capture() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    /*
    API
    CAP_ANY:자동선택
    CAP_V4L,CAP_V4L2 리눅스?
    CAP_FIREWIRE,CAP_FIREWARE,CAP_IEEE1394:IEEE 1394 드라이버
    CAP_DSHOW:다이렉트쇼
    CAP_DSHOW:Prosilica GigE SDK
    CAP_OPENNI
    CAP_MSMF
    CAP_GSTREAMER
    CAP_FFMPEG
    CAP_IMAGES:일련의 영상파일
    CAP_OPENCV_MJPEG


    컴퓨터에 카메라 한대만 입력되어 있다면 0

    VideoCapture::release():자원해제
     */
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    //사용 가능한 상태로 열렸는지 확인
    if !opened {
        panic!("Unable to open default camera!");
    }
    //동영상파일로부터 여러가지 정보를 가져오기 위해
    /*
    CAP_PROP_FRAME_WIDTH:비디오 프레임의 가로크기
    CAP_PROP_FRAME_HEIGHT:비디오 프레임의 세로크기
    CAP_PROP_FPS:초당 프레임수
    CAP_PROP_FOURCC:fource코드
    CAP_PROP_FRAME_COUNT:비디오 파일의  전체 프레임수
    CAP_PROP_BRIGHTNESS:발기조절
    CAP_PROP_CONTRAST:명암비 조절
    CAP_PROP_SATURATION:채도조절
    CAP_PROP_HUE:색상조절
    CAP_PROP_GAIN:감조조절
    CAP_PROP_EXPOSURE:노출조절
    CAP_PROP_ZOOM:줌조절
    CAP_PROP_FOCUS:초점조절
    조절=>set함수
    cap.set(CAP_PROP_POS_FRAMES,19)
     */
    println!("{}", cap.get(CAP_PROP_FRAME_WIDTH)?.round());
    println!("{}", cap.get(CAP_PROP_FRAME_HEIGHT)?.round());
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    let fps = cap.get(CAP_PROP_FPS)?;
    println!("FPS:{}", fps);
    let delay = (1000. / fps).round();
    println!("Delay{}", delay);
    println!("{}", opened);
    //일정간격마다 프레임을 받아와서 화면에 출력
    //get:여러가지 정보를 받아옴
    //set:현재열려있는 카메라 또는 비디오 파일 재생과 관련된 속성 값을 설정할떄에 사용
    loop {
        let mut frame = Mat::default();
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;
        let mut flipped_frame = frame.clone();
        /*프레임 반전 */
        // opencv::core::flip(&frame, &mut flipped_frame, 1)?;
        //c++ 의 ~같은
        opencv::core::bitwise_not(&frame, &mut flipped_frame, &no_array())?;

        if frame.size()?.width > 0 {
            highgui::imshow(window, &flipped_frame)?;
        }
        //10ms를 기다린 후 다음 프레임
        let key = highgui::wait_key(delay as i32)?;
        //27은 esc
        if key == 27 {
            break;
        }
    }
    //모든창 닫기
    destroy_all_windows()?;
    Ok(())
}

pub fn video_in() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cap = videoio::VideoCapture::from_file("girl.mp4", 0)?;
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    if !opened {
        panic!("Unable to open default capera!");
    }
    println!("{}", cap.get(CAP_PROP_FRAME_WIDTH)?.round());
    println!("{}", cap.get(CAP_PROP_FRAME_HEIGHT)?.round());
    println!("{}", cap.get(CAP_PROP_FRAME_COUNT)?.round());

    let fps = cap.get(CAP_PROP_FPS)?;
    println!("FPS:{}", fps);
    let delay = (1000.0 / fps);
    loop {
        let mut frame = Mat::default();
        if frame.empty() {
            break;
        }
        cap.read(&mut frame)?;

        let mut imversed = Mat::default();
        let mask = Mat::default(); //
        opencv::core::bitwise_not(&frame, &mut imversed, &mask)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
pub fn video_writer() -> Result<()> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    //사용 가능한 상태로 열렸는지 확인
    if !opened {
        panic!("Unable to open default camera!");
    }

    let w = cap.get(CAP_PROP_FRAME_WIDTH)? as i32;
    let h = cap.get(CAP_PROP_FRAME_HEIGHT)? as i32;
    let fps = cap.get(CAP_PROP_FPS)?;
    /*

    DIVX :DIVX코덱
    XVID MPEG-4코덱
    WMV2:windows Media Video 8코덱
    MJPG:모션 JPEG코덱
    YV12: YUV 4:2:0 Planar(비압축)
    X264 H.264코덱
    AVC1: Advanced Video코덱



     */
    // let fourcc= videoio::VideoWriter::fourcc('D', 'I', 'V', 'x')?;
    let fourcc = videoio::VideoWriter::fourcc('D', 'I', 'V', 'X')?;

    let delay = (1000.0 / fps);
    //타입에러 왁인 코닥
    /*Wait key를 눌러야 저장이됨 */
    let mut output_vedio =
        videoio::VideoWriter::new("output.mp4", fourcc, fps, Size_::new(w, h), true)?;

    //사용 가능한 상태로 열렸는지 확인
    if !output_vedio.is_opened()? {
        panic!("Unable to open default camera!");
    }

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }
        let mut inversed = Mat::default();
        opencv::core::bitwise_not(&frame, &mut inversed, &no_array())?;
        highgui::imshow("frame", &frame)?;
        highgui::imshow("inversed", &inversed)?;
        output_vedio.write(&inversed)?;
        let key = highgui::wait_key(delay as i32)?;
        if key == 27 {
            break;
        }
    }
    destroy_all_windows()?;
    Ok(())
}



pub fn video_add_capture() -> Result<()> {
    let mut cap = VideoCapture::new(0, CAP_ANY)?;

    if !cap.is_opened()? {
        println!("Camera is not opened");
        std::process::exit(0);
    }
    let mut frames = Vec::new();

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        highgui::imshow("Video display", &frame)?;
        let key = highgui::wait_key(1)?;
        if key as u8 as char == 'c' {
            frames.push(frame);
        } else if key as u8 as char == 'q' {
            break;
        }
    }
    println!("{}",frames.len());
    cap.release()?;
    highgui::destroy_all_windows()?;

    if frames.len() > 0 {
        let mut concatenated_img = frames[0].clone();
        for i in 1..frames.len().min(3) {
            println!("{}",1);

            let mut temp_img = Mat::default();
            let imgs = vec![&concatenated_img, &frames[i]];
            hconcat(&imgs[0], &mut temp_img)?;
            concatenated_img = temp_img;
        }
        highgui::imshow("Concatenated Image", &concatenated_img)?;
        highgui::wait_key(0)?;
    }
    Ok(())
}
