use opencv::core::bitwise_not;
use opencv::{
    core::{flip, Mat, MatTraitConst},
    highgui::{self, imshow, named_window, wait_key},
    imgcodecs::{self, IMREAD_GRAYSCALE},
    imgproc, Result,
};

pub fn main() -> Result<()> {
    keyboard()?;
    Ok(())
}

fn keyboard() -> Result<()> {
    let mut img = imgcodecs::imread("./img/face.jpg", IMREAD_GRAYSCALE)?;
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
        let keycode = wait_key(0)?;

        if keycode as u8 as char == 's' {
            if is_inverted {
                // 이미지 반전
                bitwise_not(&img, &mut b, &a)?;
                imshow("img", &b)?;
            } else {
                // 원래대로 되돌리기
                imshow("img", &img)?;
            }
            is_inverted = !is_inverted;
        } else if keycode as u8 as char == 'i' {
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
        }
    }
    Ok(())
}

