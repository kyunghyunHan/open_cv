use opencv::{
    core::{self, MatTraitConst}, highgui, imgcodecs,Result
    
};

pub fn main()->Result<()>{
    let mut src= core::Mat::default();
    /*imread
    filename:불러올 파일이름
    flags:영상파일 불러오기 옵션 플래그
    Mat으로 반환

    IMREAD_COLOR:컬러
    IMREAD_GRAYSCALE:그레이스 스케일 영상으로 변환하여 불러옴
     */

    /*
     imwrite
     img:저장할 영상데이터(Mat객체)
     parms:저장할 영상 파일 형식에 의존적인 파리미터   
     */
    src = imgcodecs::imread("./img/lion.jpeg", imgcodecs::IMREAD_COLOR)?;
    /*empty
    
    rows or cols 멤버가 0이거나 data멤버 변수가 null이면 true 반환
     */
    if src.empty(){
        println!("{}","image load failed");
        std::process::exit(0);
    }
    let  mut parms= core::Vector::default();
    parms.push(imgcodecs::IMWRITE_JPEG_QUALITY);
    parms.push(95);
    imgcodecs::imwrite("./img/write_test.png", &src, &parms)?;
    /*named_window 
    상단에 이름

    WINDOW_NORMAL:영상 출력청의 크기에 맞게 영상크게가 변경되어 출력
    WINDOW_AUTOSIZE:출력하는 영상 크기에 맞게 창 크기가 자동으로 변경
    WINDOW_OPENGL:OPENGL 지원
    */

    /*destoryWindow (특정창 닫기)
      destoryAll windows():e모든 창 닫기  
     */

    
    highgui::imshow("image", &src)?;
    /*
    wait_key
    사용자로부터 키도드 입력을 받는 용도로 사용

    delay
    키입력을 기다릴 시간
    0은 무한히
     */
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}