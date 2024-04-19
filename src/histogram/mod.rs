use opencv::{core::{self, no_array}, highgui, imgcodecs::{self, IMREAD_GRAYSCALE}, imgproc::calc_hist, Result};

macro_rules! greet {
    // greet! 매크로가 호출될 때 이 규칙이 적용됩니다.
    () => {
        // 매크로가 생성하는 코드
        println!("Hello, world!");
    };
    ($name:ident) => {
        $name +=1;
        println!("Hello, {}!", $name);
    };
}
pub fn main()->Result<()>{

    calc_gray_hist()?;

    
    Ok(())
}

fn calc_gray_hist()->Result<()>{
    let mut src= imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;

    let mut hist = core::Mat::default();  
    let mut num = 10;
    greet!(num);
    
    println!("{}",num);
    let channels:core::Vector<i32>= core::Vector::from_iter([0]);
    let dims= 1;
    
    let hist_size:core::Vector<i32>= core::Vector::from_iter([256]);
    let ranges:core::Vector<f32>= core::Vector::from_iter([0.,256.]);
    
    // let p_ranges: [*const f32; 1] = [ranges.as_ptr() as *const f32];
    calc_hist(&src, &channels, &no_array(), &mut hist, &hist_size, &ranges, true)?;
    highgui::imshow("src", &hist)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
  
    Ok(())
}
