use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    types,
    Result,
    imgcodecs,
};

fn main() -> Result<()> {
    let window = "face detection";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    
    let xml =core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let mut face_detector = objdetect::CascadeClassifier::new(&xml)?;

    let source_img = imgcodecs::imread("./img/car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    let mut gray = core::Mat::default();
    imgproc::cvt_color_def(&source_img, &mut gray, imgproc::COLOR_BGR2GRAY)?;

	// let mut face = objdetect::CascadeClassifier::new(&xml)?;
     

    highgui::imshow(window, &source_img)?;
    highgui::wait_key(0)?;

    Ok(())
}
