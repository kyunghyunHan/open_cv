use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, dnn, highgui, imgcodecs, imgproc, videoio, Result,types};
use opencv::prelude::NetTraitConst;
use opencv::prelude::MatTraitConst;
use opencv::prelude::VideoCaptureTrait;
use opencv::prelude::NetTrait;

pub fn main() -> Result<()> {
    let model = "./yolov8n.onnx";
  

    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if cap.is_opened()? == false {
        println!("{}", " Camera open failed!");
        std::process::exit(0);
    }

    let mut net = dnn::read_net_from_onnx(model)?;
    println!("{:?}",net);
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
            1.,
            core::Size::from((300, 300)),
            core::Scalar::default(),
            true,
            true,
            0,
        )?;
        // highgui::imshow("Object Detection", net)?;
        
        net.set_input(&blob, "", 1., core::Scalar::default())?;
        let mut net_output = types::VectorOfMat::new();
        let names = get_output_names(&net)?;

        // net.forward(&mut net_output, &names)?;
        // println!("{}", net_output.len());
        // break;
        // let detect: opencv::prelude::Mat= core::Mat::new_rows_cols_with_data(net_output.size[2], cols, typ, data, step)?;

        highgui::imshow("frame", &frame)?;
        if highgui::wait_key(1)? == 27 {
            break;
        }
    }


    Ok(())
    
}
fn get_output_names(net: &dnn::Net) -> Result<types::VectorOfString> {
    let layers = net.get_unconnected_out_layers()?;
    let layer_names = net.get_layer_names()?;

    Ok(layers
        .iter()
        .enumerate()
        .fold(types::VectorOfString::new(), |mut names, (i, _)| {
            names
                .insert(
                    i,
                    &layer_names
                        .get((layers.get(i).unwrap() - 1) as usize)
                        .expect("No such value."),
                )
                .expect("Failed inserting value.");
            names
        }))
}
