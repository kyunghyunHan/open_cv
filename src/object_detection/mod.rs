use opencv::{
    core::{self, no_array, MatTrait, MatTraitConst, Scalar},
    highgui, imgcodecs, imgproc,
    objdetect::{self, CascadeClassifierTrait, CascadeClassifierTraitConst},
    Result,
};

pub fn main() -> Result<()> {
    detect_eyes()?;
    Ok(())
}
fn template() -> Result<()> {
    let img = imgcodecs::imread("./img/circuit.bmp", imgcodecs::IMREAD_COLOR)?;
    let templ = imgcodecs::imread("./img/crystal.bmp", imgcodecs::IMREAD_COLOR)?;

    if img.empty() || templ.empty() {
        println!("{}", 1);
        std::process::exit(0);
    }

    let mut img2 = img.clone();
    core::add(
        &img,
        &core::Scalar::from((50, 50, 50)),
        &mut img2,
        &no_array(),
        0,
    )?;

    let mut noise =
        core::Mat::new_size_with_default(img.size()?, core::CV_32SC3, Scalar::default())?;

    core::randn(&mut noise, &0., &10.)?;
    let mut dst = core::Mat::default();
    core::add(&img, &noise, &mut dst, &core::Mat::default(), core::CV_8UC3)?;

    let mut res = core::Mat::default();
    let mut res_norm = core::Mat::default();

    imgproc::match_template(
        &img,
        &templ,
        &mut res,
        imgproc::TM_CCOEFF_NORMED,
        &no_array(),
    )?;
    core::normalize(
        &res,
        &mut res_norm,
        0.,
        255.,
        core::NORM_MINMAX,
        core::CV_8U,
        &no_array(),
    )?;

    let mut maxv: f64 = 0.0;
    let mut maxloc = core::Point::default();
    let mut minloc = core::Point::default();
    core::min_max_loc(
        &res,
        Some(&mut 0.),
        Some(&mut maxv),
        Some(&mut minloc),
        Some(&mut maxloc),
        &no_array(),
    )?;

    println!("maxv:{}", maxv);
    let mut img3 = img.clone();

    imgproc::rectangle(
        &mut img3,
        core::Rect_::from((maxloc.x, maxloc.y, templ.cols(), templ.rows())),
        core::Scalar::from((0, 0, 255)),
        2,
        imgproc::LINE_AA,
        0,
    )?;
    highgui::imshow("templ", &templ)?;
    highgui::imshow("res_norm", &res_norm)?;

    highgui::imshow("img", &img3)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn detect_face() -> Result<()> {
    let src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "error");
        std::process::exit(0);
    }

    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let mut classifier = objdetect::CascadeClassifier::new(&xml)?;

    if classifier.empty()? {
        println!("{}", "error");
        std::process::exit(0);
    }
    let mut faces: core::Vector<core::Rect> = core::Vector::new();
    classifier.detect_multi_scale(
        &src,
        &mut faces,
        1.3,
        1,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size_::default(),
        core::Size_::default(),
    )?;
    let mut dst = src.clone();
    for rc in faces {
        imgproc::rectangle(
            &mut dst,
            rc,
            core::Scalar::from((255, 0, 255)),
            2,
            imgproc::LINE_AA,
            0,
        )?;
    }
    highgui::imshow("src", &dst)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;
    Ok(())
}

fn detect_eyes() -> Result<()> {
    let mut src = imgcodecs::imread("./img/face.jpg", imgcodecs::IMREAD_COLOR)?;
    if src.empty() {
        println!("{}", "error");
        std::process::exit(0);
    }

    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let eye_xml = core::find_file_def("haarcascades/haarcascade_eye_tree_eyeglasses.xml")?;

    let mut face_classifier = objdetect::CascadeClassifier::new(&xml)?;
    let mut eye_classifier = objdetect::CascadeClassifier::new(&eye_xml)?;

    if face_classifier.empty()? || eye_classifier.empty()? {
        println!("{}", "error");
        std::process::exit(0);
    }
    let mut faces: core::Vector<core::Rect> = core::Vector::new();
    face_classifier.detect_multi_scale(
        &src,
        &mut faces,
        1.3,
        1,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size_::default(),
        core::Size_::default(),
    )?;
    println!("{}",faces.len());

    // let mut dst = src.clone();
    for face in faces {
        imgproc::rectangle(
            &mut src,
            face,
            core::Scalar::from((255, 0, 255)),
            2,
            imgproc::LINE_AA,
            0,
        )?;
        let mut face_roi = src.roi_mut(face)?;
       
        let mut  eyes: core::Vector<core::Rect> = core::Vector::new();

        eye_classifier.detect_multi_scale(
            &face_roi,
            &mut eyes,
            1.1,
            1,
            objdetect::CASCADE_SCALE_IMAGE,
            core::Size_::default(),
            core::Size_::default(),
        )?;
        println!("{}",eyes.len());
        for eye in eyes{
          
            let center= core::Point::from((eye.x+eye.width/2,eye.y+eye.height/2));
            imgproc::circle(&mut face_roi, center, eye.width/2, core::Scalar::from((255,0,0)), 2, imgproc::LINE_AA, 0)?;
        }
    }
    highgui::imshow("src", &src)?;
    highgui::wait_key(0)?;
    highgui::destroy_all_windows()?;

    Ok(())
}
fn hog()->Result<()>{
    Ok(())
}