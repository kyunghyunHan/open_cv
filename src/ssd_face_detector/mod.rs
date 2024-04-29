use opencv::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;
use opencv::prelude::NetTrait;
use opencv::prelude::NetTraitConst;
use opencv::prelude::VideoCaptureTrait;
use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, dnn, highgui, imgcodecs, imgproc, types, videoio, Result};

pub fn main() -> Result<()> {
    let model = "./dataset/res10_300x300_ssd_iter_140000_fp16.caffemodel";
    let config = "./dataset/deploy.prototxt";

    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if cap.is_opened()? == false {
        println!("{}", " Camera open failed!");
        std::process::exit(0);
    }

    let mut net = dnn::read_net(model, config, "")?;
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
            1.0/255.0,
            core::Size::from((512, 512)),
            core::Scalar::from((104, 177, 123)),
            false,
            false,
            core::CV_32F,
        )?;
        // highgui::imshow("Object Detection", net)?;

        net.set_input(&blob, "", 1.0, core::Scalar::from((0,0,0)))?;
        let mut net_output = types::VectorOfMat::new();
        net.forward(&mut net_output, &net.get_unconnected_out_layers_names()?)?;
        
        let res = net_output.get(0)?;

        let rows = *res.mat_size().get(1).unwrap();

       
  
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }

    Ok(())
}
