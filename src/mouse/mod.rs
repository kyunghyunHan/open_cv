use opencv::{
    core::{self, Mat, Point, Point_, Scalar, ToInputOutputArray},
    highgui::{self, imshow, set_mouse_callback, wait_key, EVENT_LBUTTONDOWN, EVENT_LBUTTONUP, EVENT_MOUSEMOVE, WINDOW_AUTOSIZE},
    imgcodecs::{self, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc,
    Result,
};
use std::sync::{Arc, Mutex};
/*Mouse Event
EVENT_MOUSEMOVE:마우스가 창 위에서 움직이는 경우
EVENT_LBUTTONDOWN:마우스 왼쪽버튼 누른경우
EVENT_RBUTTONDOWN:마우스 오른쪽버튼 누른경우
EVENT_MBUTTONDOWN:마우스 가운데버튼 누른경우
EVENT_LBUTTONUP:마우스 왼쪽버튼 뗸경우
EVENT_RBUTTONUP:마우스 오른쪽버튼 뗸경우
EVENT_MBUTTONUP:마우스 가운데버튼 뗸경우
EVENT_LBUTTONDBLCLK:마우스 왼쪽버튼 더블클릭
EVENT_RBUTTONDBLCLK:마우스 오른쪽버튼 더블클릭
EVENT_MBUTTONDBLCLK:마우스 가운데버튼 더블클릭
EVENT_MOUSEWHEEL:마우스 휠 앞뒤
EVENT_MOUSEHWHEEL:마우스휠좌우

*/

/*
Flag Event

EVENT_FLAG_LBUTTON :마우스 왼쪽버튼이 눌려있음
EVENT_FLAG_RBUTTON :마우스오른쪽버튼이 눌려있음
EVENT_FLAG_MBUTTON :마우스 가운데버튼이 눌려있음
EVENT_FLAG_CRTLKEY : ctrl버튼이 눌려있음
EVENT_FLAG_SHIFTKEY shift 왼쪽버튼이 눌려있음
EVENT_FLAG_ALTKEY :ALT 왼쪽버튼이 눌려있음


*/

pub fn main() -> Result<()> {
    let  img: Arc<Mutex<Mat>> = Arc::new(Mutex::new(imgcodecs::imread("./img/face1.jpeg", IMREAD_COLOR)?));
  
    highgui::named_window("img", WINDOW_AUTOSIZE)?;

    let img_clone = Arc::clone(&img);
    let pt_old = Arc::new(Mutex::new(Point_::default()));

    set_mouse_callback("img", Some(Box::new(move |event, x, y, _flag| {
        let mut img_guard = img_clone.lock().unwrap();
        let mut pt_old_guard = pt_old.lock().unwrap();
        
        match event {
            EVENT_LBUTTONDOWN => {
                *pt_old_guard = Point_::from((x, y));
                println!("EVENT_LBUTTONDOWN: {},{}", x, y);
            }
            EVENT_LBUTTONUP => {
                
                println!("EVENT_LBUTTONUP: {},{}", x, y);
                
            }
            EVENT_MOUSEMOVE => {
                if _flag == EVENT_LBUTTONDOWN{
                           let color = Scalar::from((0, 255, 0)); // BGR 순서로 색상을 지정합니다. 여기서는 녹색입니다.
                           let thickness = 2;
                imgproc::line(&mut *img_guard, *pt_old_guard, Point_::from((x, y)), color, thickness, imgproc::LINE_8, 0).unwrap();
                imshow("img", &mut *img_guard).unwrap();
                *pt_old_guard = Point_::from((x, y));
                }
         
            }
            _ => {}
        }
    })))?;

    imshow("img", &*img.lock().unwrap())?;

    wait_key(0)?;

    Ok(())
}
