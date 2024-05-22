use opencv::{
    core, dnn, imgcodecs, imgproc,
    prelude::{MatTraitConst, NetTraitConst,NetTrait},
    Result,
};

pub fn main() -> Result<()> {
    let model = "./dataset/model.onnx";
    let image_path = "./img/food5.jpeg"; //이미지 경로
    let frame = imgcodecs::imread(&image_path, imgcodecs::IMREAD_COLOR)?;
    let mut net = dnn::read_net_from_onnx(model)?;
    if net.empty()? {
        println!("{}", "Net Open Failed");
        std::process::exit(0);
    }

    if frame.empty() {
        println!("{}", "Frame Open Failed");
        std::process::exit(0);
    }
    let mut umat: core::UMat = core::UMat::new_def();
    frame.copy_to(&mut umat)?;
    println!("{:?}", umat);
    let blob = dnn::blob_from_image(
        &umat,
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

    let rows = *res.mat_size().get(1).unwrap(); 
    let cols = *res.mat_size().get(0).unwrap(); 
    let mut max_class_idx = 0; // 최대 확률을 가진 클래스의 인덱스를 저장합니다.
    let mut max_probability = 0.0; // 최대 확률을 저장합니다.
    
    for class_idx in 0..5 { // Q1부터 Q5까지의 클래스에 대해 반복합니다.
        let class_probability = *res.at_2d::<f32>(0, class_idx)?; // 클래스의 확률을 가져옵니다.
        if class_probability > max_probability { // 현재 클래스의 확률이 최대 확률보다 높으면
            max_probability = class_probability; // 최대 확률을 업데이트합니다.
            max_class_idx = class_idx; // 최대 확률을 가진 클래스의 인덱스를 업데이트합니다.
        }
    }
    println!("Highest probability class: Q{}", max_class_idx + 1); // 가장 높은 확률을 가진 클래스를 출력합니다.
    println!("Probability: {}", max_probability); // 해당 클래스의 확률을 출력합니다
    Ok(())
}
