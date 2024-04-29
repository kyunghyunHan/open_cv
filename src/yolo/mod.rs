use opencv::prelude::MatExprTraitConst;
use opencv::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;
use opencv::prelude::NetTrait;
use opencv::prelude::NetTraitConst;
use opencv::prelude::VideoCaptureTrait;
use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, dnn, highgui, imgcodecs, imgproc, types, videoio, Result};
pub fn main() -> Result<()> {
    let model = "./yolov8n.onnx";
    let config = "./dataset/deploy.prototxt";

    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if cap.is_opened()? == false {
        println!("{}", " Camera open failed!");
        std::process::exit(0);
    }

    let mut net = dnn::read_net_from_onnx(model)?;
    // let out_names = net.get_unconnected_out_layers_names().unwrap();

    println!("{:?}", net);
    if net.empty()? {
        println!("{}", "Net Open Failed");
        std::process::exit(0);
    }

    loop {
        let mut frame = core::Mat::default();
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;

        if frame.empty() {
            println!("{}", "Net Open Failed");
            std::process::exit(0);
        }

        let blob = dnn::blob_from_image(
            &frame,
            1.0 / 255.0,
            core::Size::from((640, 640)),
            core::Scalar::from((0., 0., 0., 0.)),
            true,
            false,
            core::CV_32F,
        )?;
        let out_layer_names = net.get_unconnected_out_layers_names()?;
        let mut net_output: core::Vector<core::Mat> = core::Vector::new();
        net.set_input(&blob, "", 1.0, core::Scalar::default())?;
        net.forward(&mut net_output, &out_layer_names)?;

        let res = net_output.get(0)?;

        // let rows = *res.mat_size().get(1).unwrap();
        let rows = *res.mat_size().get(2).unwrap(); // 8400

        println!("{}", rows);
        let cols = *res.mat_size().get(1).unwrap(); // M
        println!("{}", cols);
        let mut boxes: core::Vector<core::Rect> = core::Vector::default();
        let mut scores: core::Vector<f32> = core::Vector::default();
        let mut indices: core::Vector<i32> = core::Vector::default();
        let mut class_index_list: core::Vector<i32> = core::Vector::default();

        // for row in 0..rows(){

        // }
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    Ok(())
}
// yolo.rs
fn pre_process(img: &core::Mat) -> opencv::Result<core::Mat> {
    let width = img.cols();
    let height = img.rows();

    let _max = std::cmp::max(width, height);
    // keep the original aspect ratio by adding black padding
    let mut result = core::Mat::zeros(_max, _max, core::CV_8UC3)
        .unwrap()
        .to_mat()
        .unwrap();
    img.copy_to(&mut result)?;

    Ok(result)
}
