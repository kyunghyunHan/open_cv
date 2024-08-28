use opencv::{
    core::{
        self, bitwise_not, flip, no_array, Mat, Point, Point_, Scalar, ToInputOutputArray, CV_8UC1,
    },
    highgui::{
        self, create_trackbar, destroy_window, imshow, named_window, set_mouse_callback, wait_key,
        EVENT_LBUTTONDOWN, EVENT_LBUTTONUP, EVENT_MOUSEMOVE, WINDOW_AUTOSIZE,
    },
    imgcodecs::{self, imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc,
    prelude::*,
    Result,
};
use std::sync::{Arc, Mutex};

/*keyboard
opencv에서 키입력을 확인하기 위해 사용하는 함수는 waitKey()이며 waitkey함수는 delay에 해당하는 밀리초 시간동안 키입력을 기다리다가 키입력이 있으면 해당 키의 아스키코드 값을 반환합니다.
만약 지정한 시간동안 키입력이 없으면 waitKey()함수는 -1을 반환합니다.함수의 인자를 지정하지 않거나 0또는 음수로 설정하게 되면 키입력이 있을떄까지 무한이 기다리며 사용자가 키보드를 누르면 누린키의 아스키 코드를 반환하면서 함수가 종료합니다.
만약 특정 키를 눌렀을 떄에만 영상 출력창을 닫게끔 만들려면 waitKey()함수의 반환값을 조사해야합니다.
ESc키를 누를떄 창을닫게하려면 다음과같이 작성하면 됩니다.


loop을 돌며 if조건문을 사용하여 waitKey()가 27이면 빠져나오게 설정되있습니다.27 은 ESC키를 의미합니다.

다음은 이미지를 s 키를  누를떄마다 영상의 이미지를 반전해서 보여줍니다.
i를 누를경우 좌우 반전을 시켜주며 q를 누르면 종료하게 됩니다.


*/

/*Mouse Event
마우스 이벤트는 마우스 클릭에 반응하거나 마우스를 드래그 하여 영상에  그림을 그리는 등의 동작을 수행할수 있습니다.
마우스 이벤트를 처리하려면 마우스 콜백함수를 사용하여 이벤트를 처리해야합니다.
특정창의 마우스 콜백함수를 등록할 떄에는 set_mouse_callback()를 사용합니다.

winname:마우스 이벤터 처리를 할 창의 이름
on_mouse:마우스 이벤트 처리를 위한 콜백 함수 이름
userdata:콜백 함수애 전달할 사용자 데이터의 포인터


winname창에서 마우스 이벤트가 발생하게 되면 on_mouse로 등록된 콜백함수가 자동으로 호출되도록 설정합니다.
마우스 콜백함수는 네개의정수형과 하나의 void*타입을 인자로 가지며 void를 반환형으로 설정해야합니다.


MouseEventType열거형 상수


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


이예는 마우스 왼쪽버튼이 눌리거나 떼어진 좌표를 콘솔창에 출력하고 마우스 왼쪽버튼을 누른상태로 마우스를 움직이면 초록색으로 라인을 그릴수 있습니다.
on_mouse()함수는 Rust의 match문을 사용하여 다양한 마우스 처리를 할수 있습니다.
예제는 다음과 같습니다.

trackbar


Opencv에는 GUI를 추가하여 트랙바인터페이스를 사용할수 있습니다.트랙바는 슬라이서 컨트롤 이라고도 불리며 영상 출력창에 부착되어 프로그램 동작 중에 사용자가 지정된 범위안의 값을 선택할수 있습니다.
트랙바는 사용자가 지정한 영상 출력창에 상단에 부착되며 필요한 경우 창 하나에 여러개의 트랙바를 생성할수 있습니다.각각의 트랙바에는 고유한 이름설정해야하며 생성시 최소 위치는 항상 0으로 고정되어 있습니다.

생성하려면 crate_trackbar() 함수를 사용해야하며 다음과같습니다.
trackbarname:
winname:
value:
count:
on_change:
useradta:
반환값:

*/

fn keyboard_event_exampe() -> Result<()> {
    let img = imread("./img/face1.jpeg", IMREAD_COLOR)?;

    named_window("img", 0)?;
    imshow("img", &img)?;
    loop {
        if wait_key(-1)? == 27 {
            break;
        }
    }
    destroy_window("img")?;
    Ok(())
}
fn mouse_event() -> Result<()> {
    let img: Arc<Mutex<Mat>> = Arc::new(Mutex::new(imgcodecs::imread(
        "./img/face1.jpeg",
        IMREAD_COLOR,
    )?));

    highgui::named_window("img", WINDOW_AUTOSIZE)?;

    let img_clone = Arc::clone(&img);
    let pt_old = Arc::new(Mutex::new(Point_::default()));

    set_mouse_callback(
        "img",
        Some(Box::new(move |event, x, y, _flag| {
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
                EVENT_MOUSEMㅂOVE => {
                    if _flag == EVENT_LBUTTONDOWN {
                        let color = Scalar::from((0, 255, 0)); // BGR 순서로 색상을 지정합니다. 여기서는 녹색입니다.
                        let thickness = 2;
                        imgproc::line(
                            &mut *img_guard,
                            *pt_old_guard,
                            Point_::from((x, y)),
                            color,
                            thickness,
                            imgproc::LINE_8,
                            0,
                        )
                        .unwrap();
                        imshow("img", &mut *img_guard).unwrap();
                        *pt_old_guard = Point_::from((x, y));
                    }
                }
                _ => {}
            }
        })),
    )?;

    imshow("img", &*img.lock().unwrap())?;

    wait_key(0)?;

    Ok(())
}

fn keyboard_event() -> Result<()> {
    let mut img = imgcodecs::imread("./img/face1.jpeg", IMREAD_GRAYSCALE)?;
    if img.empty() {
        println!("{}", "image load failed");
        std::process::exit(0);
    }

    named_window("img", 0)?;
    imshow("img", &img)?;
    let a = Mat::default();
    let mut b = Mat::default();
    let mut is_flipped = false; // 이미지가 반전되었는지 여부를 나타내는 플래그
    let mut is_inverted = false;

    loop {
        if wait_key(0)? as u8 as char == 's' {
            println!("{}", "이미지 반전");
            if is_inverted {
                // 이미지 반전
                bitwise_not(&img, &mut b, &a)?;
                imshow("img", &b)?;
            } else {
                // 원래대로 되돌리기
                imshow("img", &img)?;
            }
            is_inverted = !is_inverted;
        } else if wait_key(0)? as u8 as char == 'i' {
            println!("{}", "좌우반전");
            // 이미지가 반전되었는지 확인하고 반전되지 않았다면 반전시키고, 이미 반전되었다면 다시 원래대로 반전시킴
            if !is_flipped {
                flip(&img, &mut b, 1)?;
                imshow("img", &b)?;
                is_flipped = true;
            } else {
                flip(&b, &mut img, 1)?;
                imshow("img", &img)?;
                is_flipped = false;
            }
        } else if wait_key(0)? as u8 as char == 'q' {
            break;
        }
    }
    Ok(())
}

fn trackbar_event() -> Result<()> {
    let img: Arc<Mutex<Mat>> = Arc::new(Mutex::new(
        Mat::zeros(400, 400, CV_8UC1).unwrap().to_mat().unwrap(),
    ));

    let img_clone = Arc::clone(&img);
    named_window("image", WINDOW_AUTOSIZE)?;
    create_trackbar(
        "level",
        "image",
        None,
        16,
        Some(Box::new(move |pos: i32| {
            let mut img_guard = img_clone.lock().unwrap();
            img_guard
                .set_to(&Scalar::from(pos * 16), &no_array())
                .unwrap();
            imshow("image", &*img_guard).unwrap();
        })),
    )?;
    imshow("image", &*img.lock().unwrap())?;

    wait_key(0)?;

    Ok(())
}
pub fn main() -> Result<()> {
    // keyboard_event()?;
    // mouse_event()?;
    trackbar_event()?;
    Ok(())
}
