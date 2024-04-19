use opencv::{core, dnn, highgui, prelude::*, videoio, Result};

pub fn main() -> Result<()> {
    let mut net = dnn::read_net_from_onnx("./best.onnx")?;
    net.set_preferable_backend(dnn::DNN_BACKEND_DEFAULT)?;
    net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

    let out_names = net.get_unconnected_out_layers_names().unwrap();

    let window = "video capture";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera

    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut outs = core::Vector::<core::Mat>::new(); //core::Mat::default()?;
    let mut outs = core::Vector::<core::Mat>::new(); //core::Mat::default()?;
    

    loop {
        let mut frame = Mat::default();

        cam.read(&mut frame)?;
        if frame.size()?.width > 0 && frame.size()?.height > 0 {
            let blob = dnn::blob_from_image(
                &frame,
                1.,
                core::Size::new(640,640), // 이미지 크기를 조정하여 모델에 맞게 변경
                core::Scalar::new(0.0, 0.0, 0.0, 0.0),
                true,
                false,
                core::CV_32F,
            )?;
            // 모델에 입력하고 결과 받아오기
			let scale = 1.0 / 255.0;
			let mean = (0.0, 0.0, 0.0);

			net.set_input(&blob, "", scale, mean.into())?;
            net.forward(&mut outs, &out_names)?;

            // 결과를 처리하여 bounding box 그리기 등의 작업 수행
            // 이 부분은 모델에 따라 출력 형식이 달라지므로 모델에 맞게 수정이 필요합니다.
            for i in 0..outs.len() {
                let detection = outs.get(i)?;
                println!("Detection {}: {:?}", i, detection);
            }
            highgui::imshow(window, &frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}
  