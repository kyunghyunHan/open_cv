use opencv::{core, highgui, imgcodecs, imgproc, Result,videoio};
pub fn main() -> Result<()> {
    klt_algorithm()?;
    Ok(())
}
/*추적 */
fn klt_algorithm()->Result<()>{

    let cap = videoio::VideoCapture::from_file("./",videoio::CAP_ANY)?;
    
    Ok(())
}