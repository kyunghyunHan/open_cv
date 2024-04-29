use opencv::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;
use opencv::prelude::NetTrait;
use opencv::prelude::NetTraitConst;
use opencv::prelude::VideoCaptureTrait;
use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, dnn, highgui, imgcodecs, imgproc, types, videoio, Result};
pub fn main() -> Result<()> {
    let model = "./res10_300x300_ssd_iter_140000_fp16.caffemodel";
    let config = "./deploy.prototxt";

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
        println!("{}",names.len());
        net.forward(&mut net_output, &names)?;
        let nms_threshold = 0.4_f32;
        let conf_threshold = 0.5_f32;
        let mut class_ids = types::VectorOfi32::new();
        let mut confidences = types::VectorOff32::new();
        let mut boxes = types::VectorOfRect::new();
        println!("{}",net_output.len());
        for (i, matrix) in net_output.iter().enumerate() {
            for j in 0..matrix.rows() {
                println!("{}","11111");

                let img_width = frame.cols();
                let img_height = frame.rows();
                let data = matrix.at_row::<f32>(j as i32)?;
                let scores = net_output.get(i)?;
                let scores = scores.row(j)?;
                let scores = scores.col_range(&core::Range::new(5, net_output.get(i)?.cols())?)?;
                let mut class_id_point = core::Point::default();
                let mut confidence = 0_f64;

                core::min_max_loc(
                    &scores,
                    Some(&mut 0.),
                    Some(&mut confidence),
                    Some(&mut core::Point::new(0, 0)),
                    Some(&mut class_id_point),
                    &core::no_array(),
                )?;

                if confidence > conf_threshold as f64 {
                    let center_x = (data[0] * img_width as f32) as i32;
                    let center_y = (data[1] * img_height as f32) as i32;
                    let width = (data[2] * img_width as f32) as i32; // w
                    let height = (data[3] * img_height as f32) as i32; // h
                    let left = center_x - (width / 2); // x
                    let top = center_y - (height / 2); // y

                    class_ids.push(class_id_point.x);
                    confidences.push(confidence as f32);
                    boxes.push(core::Rect::new(left, top, width, height));
                }
                let mut indices = types::VectorOfi32::new();
                println!("{}",indices.len());
                dnn::nms_boxes(&boxes, &confidences, conf_threshold, nms_threshold, &mut indices, 1., 0)?;
                for num in indices.iter() {
                    // get bounding box and associated class
                    let bbox = boxes.get(num as usize)?;
                    let label = names.get(class_ids.get(num as usize)? as usize)?;

                    // draw predicted bounding box with associated class
                    imgproc::rectangle(
                        &mut frame,
                        bbox,
                        core::Scalar::new(255., 18., 50., 0.0),
                        2,
                        imgproc::LINE_8,
                        0,
                    )?;
                    imgproc::put_text(
                        &mut frame,
                        &label,
                        core::Point::new(bbox.x, bbox.y),
                        imgproc::FONT_HERSHEY_SIMPLEX,
                        0.75,
                        core::Scalar::new(0., 0., 0., 0.),
                        1,
                        imgproc::LINE_8,
                        false,
                    )?;
                }
            }
        }
        // let detect= core::Mat::new_rows_cols_with_default(res.size()?.width, res.size()?.height, core::CV_32FC1, r)?;
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
