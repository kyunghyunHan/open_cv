use opencv::{core, highgui, imgcodecs, imgproc, ml, Result};
/*머신러닝
학습 = train
결과 예측=>predict,inference
*/
pub fn main() -> Result<()> {
    Ok(())
}
fn knn_plane() -> Result<()> {
    let img = core::Mat::zeros(500, 500, core::CV_8UC3)?;
    let knn = ml::KNearest::create()?;
    let mut k_value = 1;
    highgui::named_window("knn", 0)?;
    highgui::create_trackbar(
        "k",
        "knn",
        Some(&mut k_value),
        5,
        Some(Box::new({ |x| {


            
        } })),
    )?;

    Ok(())
}
