use opencv::core::{no_array, MatExprTraitConst};
use opencv::{
    prelude::MatTrait,
    core::{Mat, CV_8UC1,Scalar},
    highgui::{create_trackbar, named_window, WINDOW_AUTOSIZE,imshow,wait_key},
    Result,
};

use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> Result<()> {
    let img: Arc<Mutex<Mat>> = Arc::new(Mutex::new(
        Mat::zeros(400, 400, CV_8UC1).unwrap().to_mat().unwrap(),
    ));

    // let img_clone = Arc::clone(&img);
    let img_clone = Arc::clone(&img);

    named_window("image", WINDOW_AUTOSIZE)?;
    create_trackbar(
        "level",
        "image",
        None,
        16,
        Some(Box::new(move |pos: i32| {

            let mut img_guard = img_clone.lock().unwrap();
            // img_guard.set_to(pos,&no_array()).unwrap();

            imshow("image", &*img_guard).unwrap();


        })),
    )?;
    // imshow("image", &img.lock().unwrap()).unwrap();
    imshow("img", &*img.lock().unwrap())?;

    wait_key(10000)?;
    
    Ok(())
}
