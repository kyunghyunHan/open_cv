use opencv::{
    core::{self, MatExprTraitConst},
    highgui, imgcodecs, imgproc, ml, Result,
};
use std::sync::{Arc, Mutex};
/*머신러닝
학습 = train
결과 예측=>predict,inference
*/
pub fn main() -> Result<()> {
    Ok(())
}
fn knn_plane() -> Result<()> {
    let img = Arc::new(Mutex::new(
        core::Mat::zeros(500, 500, core::CV_8UC3)?.to_mat()?,
    ));
    let img_clone = Arc::clone(&img);
    let knn = ml::KNearest::create()?;
    let mut k_value = 1;
    highgui::named_window("knn", 0)?;
    highgui::create_trackbar(
        "k",
        "knn",
        Some(&mut k_value),
        5,
        Some(Box::new({
            move |x| {
                let img_guard = img_clone.lock().unwrap();
            }
        })),
    )?;
    let num = 30;
    let mut rn = core::Mat::new_rows_cols_with_default(num, 2, core::CV_32SC1, core::Scalar::default())?;

    core::randn(&mut rn, &0., &50.)?;

    for i in 0..num{
     //addPoint
    }
    Ok(())
}
