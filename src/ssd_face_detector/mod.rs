use opencv::core::{Scalar, Size};
use opencv::dnn::{self,blob_from_image, NetTrait};
use opencv::prelude::VideoCaptureTrait;
use opencv::types::VectorOfMat;
use opencv::types::VectorOfString;
use opencv::{
    core::{Mat, MatTraitConst},
    dnn::{read_net, NetTraitConst},
    videoio::{self, VideoCapture, VideoCaptureTraitConst},
    Result,
};
pub fn main() -> Result<()> {
    let model = "./dataset/res10_300x300_ssd_iter_140000_fp16.caffemodel";
    let config = "./dataset/deploy2.prototxt";

    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if cap.is_opened()? == false {
        println!("{}", " Camera open failed!");
        std::process::exit(0);
    }

    let mut net = read_net(model, config, "")?;

    if net.empty()? {
        println!("{}", "Net Open Failed");
        std::process::exit(0);
    }

    loop {
        let mut frame = Mat::default();
        //카메라 또는 동영상 파일로 부터 다음 프레임을 받아와서 MAt클랙스 형식의 변수 이미지에 저장
        cap.read(&mut frame)?;

        if frame.empty() {
            println!("{}", "Net Open Failed");
            std::process::exit(0);
        }

        let blob = blob_from_image(
            &frame,
            1.,
            Size::from((300, 300)),
            Scalar::from((104, 177, 123)),
            false,
            false,
            0,
        )?;
        net.set_input(&blob, "data", 1., Scalar::default())?;
        let mut net_output = VectorOfMat::new();
        let names = get_output_names(&net)?;

        net.forward(&mut net_output, &names)?;
        println!("{}", net_output.len());
        break;
        // let detect= Mat::new_rows_cols_with_data(net_output.size[2], cols, typ, data, step);
    }

    Ok(())
}
fn get_output_names(net: &dnn::Net) -> Result<VectorOfString> {
    let layers = net.get_unconnected_out_layers()?;
    let layer_names = net.get_layer_names()?;

    Ok(layers
        .iter()
        .enumerate()
        .fold(VectorOfString::new(), |mut names, (i, _)| {
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
