use opencv::core::{hconcat, Vector};
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_PROP_FPS};
use opencv::{
    core::{bitwise_not, no_array, Size_},
    highgui::{self, destroy_all_windows, wait_key},
    prelude::*,
    videoio::{self, CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH},
    Result,
};
/*Video Capture

Video Capture 클래스 => 카메라 또는 동영상 피일로 부터 정지 영상 프레임임을 받아올떄 사용합니다
동영상에 저장되어 있는 일련의 정지 영상을 프레임 이라 하며 동영상을 처리하는 작업은 동영상에서 프레임을 추출한 후 각각의 프레임에 영상 처리 기법을을 적용하는 형태로 이루어짐

api preference 인자에는 동영상 파일을 불러오는 방법을지정할수 있음
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



동영상 여러 정보 가져오기

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


비디오 코덱
    DIVX :DIVX코덱
    XVID MPEG-4코덱
    WMV2:windows Media Video 8코덱
    MJPG:모션 JPEG코덱
    YV12: YUV 4:2:0 Planar(비압축)
    X264 H.264코덱
    AVC1: Advanced Video코덱
*/
pub fn main() -> Result<()> {
    // camera_in()?;
    // video_in()?;
    camera_in_video_out()?;
    // video_add_capture()?;
    Ok(())
}
pub fn camera_in() -> Result<()> {


    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    //컴퓨터에 카메라 한대만 입력되어 있다면 index 0

   //file name에는 "mp4,.avi등 동영상 파일 이름을 전달 하나의 동영상 파일대신 일련의 숫자로 구분되는 이름의 정지영상을 가지고 있고  이파일을 차례대로 불러오고싶을떄도 사용가능
    //예를 들어 img0001.jpg , img0002.jpg , img0003.jpg 등 차례대로 불러오고싶다면  인자에 img%04d.jpg라고 입력하여 일련의 영상을 차례대로  불러오는게 가능
    //" "protocol://host:port" 형태의 비디오 스트림 URL을 지정하여 인터넷 동영상도 사용가능
    //index가 0이면  첫번쨰 카메라 장치를 사용한다는 것
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    //사용 가능한 상태로 열렸는지 확인
    //자원해제

    //is opened 는 카메라 장치가 사용하면 true 그렇지 않으면 false
    if !cap.is_opened()? {
        panic!("Unable to open default camera!");
    }
    //카메라 장치 또는 동영상 파일의 사용이 끝나면 자원을 release함수를 호출하여 자원을 해제해야함
    // cap.release()?;//
    println!("Frame width:{}", cap.get(CAP_PROP_FRAME_WIDTH)?.round());
    println!("Frame height{}", cap.get(CAP_PROP_FRAME_HEIGHT)?.round());
    
    let fps = cap.get(CAP_PROP_FPS)?;
    println!("FPS:{}", fps);
    let delay = (1000. / fps).round();
    println!("Delay{}", delay);

    //일정간격마다 프레임을 받아와서 화면에 출력
    //get:여러가지 정보를 받아옴
    //set:현재열려있는 카메라 또는 비디오 파일 재생과 관련된 속성 값을 설정할떄에 사용
    let mut frame = Mat::default();
    let mut inversed = Mat::default();
    loop {
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;

        /*프레임 반전 */
        // opencv::core::flip(&frame, &mut flipped_frame, 1)?;
        //c++ 의 ~같은
        opencv::core::bitwise_not(&frame, &mut inversed, &no_array())?;
        highgui::imshow("frame", &frame)?;
        highgui::imshow("inversed", &inversed)?;

        //10ms를 기다린 후 다음 프레임 받아오게댐
        //27은 esc 
        //키값이 27이면 while루프를 빠져나오도록 설게
        if wait_key(10 as i32)? == 27 {
            break;
        }
    }
    //모든창 닫기
    destroy_all_windows()?;
    Ok(())
}

pub fn video_in() -> Result<()> {
    //대부분의 동영상 파일은고유의 코덱을 이용하여 압축한 형태로 저장
    let mut cap = videoio::VideoCapture::from_file("./video/face2.mp4", CAP_ANY)?;
    if !cap.is_opened()? {
        panic!("Unable to open default capera!");
    }
    //동영상의 정보 가져오기
    println!("{}", cap.get(CAP_PROP_FRAME_WIDTH)?.round());
    println!("{}", cap.get(CAP_PROP_FRAME_HEIGHT)?.round());
    println!("{}", cap.get(CAP_PROP_FRAME_COUNT)?.round());

    //초당 30프레임을 재생하는 경우 delay값은 33이며 이는 매 프레임을 33ms시간 간격으로 출력해야 함을 의미함
    //여기서 구한 delay는 이후 동영상 프레임을 받아 화면에 출력하는 반복문안에서 waitKey()함수의 인자로 사용
    let fps = cap.get(CAP_PROP_FPS)?;
    println!("FPS:{}", fps);

    let delay = (1000. / fps).round();
    let mut frame = Mat::default();
    let mut inversed = Mat::default();

    loop {
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }
        opencv::core::bitwise_not(&frame, &mut inversed, &no_array())?;
        highgui::imshow("frame", &frame)?;
        highgui::imshow("inversed", &inversed)?;

        //27은 esc
        if wait_key(delay as i32)? == 27 {
            break;
        }
    }
    destroy_all_windows()?;
    Ok(())
}

pub fn camera_in_video_out() -> Result<()> {

    //opencv 에서 동영상 파일을 생성하려면 Video Writer클래스 사용
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    //사용 가능한 상태로 열렸는지 확인
    if !cap.is_opened()? {
        panic!("Unable to open default camera!");
    }

    let w = cap.get(CAP_PROP_FRAME_WIDTH)? as i32;
    let h = cap.get(CAP_PROP_FRAME_HEIGHT)? as i32;
    let fps = cap.get(CAP_PROP_FPS)?;
    //코덱설정
    let fourcc = videoio::VideoWriter::fourcc('X', '2', '6', '4')?; //m1기준 x264=>H.264

    let delay = 1000.0 / fps.round();
    //타입에러 왁인 코닥
    /*Wait key를 눌러야 저장이됨 */
    let mut output_vedio =
        videoio::VideoWriter::new("./video/output.mp4", fourcc, fps, Size_::new(w, h), true)?;

    //사용 가능한 상태로 열렸는지 확인
    if !output_vedio.is_opened()? {
        panic!("Unable to open default camera!");
    }
    let mut frame = Mat::default();
    let mut inversed = Mat::default();

    loop {
        cap.read(&mut frame)?;
        if frame.empty() {
            break;
        }
        bitwise_not(&frame, &mut inversed, &no_array())?;
        highgui::imshow("frame", &frame)?;
        highgui::imshow("inversed", &inversed)?;
        output_vedio.write(&inversed)?;
        if wait_key(delay as i32)? == 27 {
            break;
        }
    }
    destroy_all_windows()?;
    Ok(())
}

pub fn video_add_capture() -> Result<()> {
    let mut cap = VideoCapture::new(0, CAP_ANY)?;

    if !cap.is_opened()? {
        panic!("Unable to open default camera!");
    }
    let mut frames: Vector<Mat> = Vector::new();

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        highgui::imshow("비디오 디스플레이", &frame)?;
        let key = highgui::wait_key(1)?;
        if key as u8 as char == 'c' {
            frames.push(frame);
        } else if key as u8 as char == 'q' {
            break;
        }
    }
    cap.release()?;
    highgui::destroy_all_windows()?;

    if frames.len() > 0 {
        let mut imgs: Vector<Mat> = Vector::new();
        for i in 0..frames.len().min(3) {
            imgs.push(frames.get(i)?.clone());
        }
        let mut concatenated_image = Mat::default();
        hconcat(&imgs, &mut concatenated_image)?;

        highgui::imshow("병합된 이미지", &concatenated_image)?;
        highgui::wait_key(0)?;
    }
    Ok(())
}
