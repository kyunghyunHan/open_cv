use opencv::prelude::MatTraitConst;
use opencv::prelude::NetTrait;
use opencv::prelude::NetTraitConst;
use opencv::prelude::UMatTraitConst;
use opencv::prelude::VideoCaptureTrait;
use opencv::prelude::VideoCaptureTraitConst;
use opencv::{core, dnn, highgui, imgcodecs, imgproc, types, videoio, Result};

pub fn main() -> Result<()> {
    let model = "./dataset/best.onnx";
    let image_path = "./img/test.jpg"; //이미지 경로
     
    let mut frame = imgcodecs::imread(&image_path, imgcodecs::IMREAD_COLOR)?;

    let mut net = dnn::read_net_from_onnx(model)?;

    if net.empty()? {
        println!("{}", "Net Open Failed");
        std::process::exit(0);
    }



    if frame.empty() {
        println!("{}", "Net Open Failed");
        std::process::exit(0);
    }

    let mut umat: core::UMat  = core::UMat::new_def();
    frame.copy_to(&mut umat)?;
    println!("{:?}",umat);
    let blob = dnn::blob_from_image(
        &umat,
        1.0 / 255.0,
        core::Size::from((640, 640)),
        core::Scalar::from((0., 0., 0., 0.)),
        true,
        false,
        core::CV_32F,
    )?;
    println!("{}",1);

    let out_layer_names = net.get_unconnected_out_layers_names()?;
    let mut net_output: core::Vector<core::Mat> = core::Vector::new();
    net.set_input(&blob, "", 1.0, core::Scalar::default())?;
    net.forward(&mut net_output, &out_layer_names)?;
    let res = net_output.get(0)?;
    let rows = *res.mat_size().get(2).unwrap(); // 8400
    let cols = *res.mat_size().get(1).unwrap(); // M
    let mut boxes: core::Vector<core::Rect> = core::Vector::default();
    let mut scores: core::Vector<f32> = core::Vector::default();
    let mut indices: core::Vector<i32> = core::Vector::default();
    let mut class_index_list: core::Vector<i32> = core::Vector::default();
    let x_scale = umat.cols() as f32 / 640f32;
    let y_scale = umat.rows() as f32 / 640f32;

    for row in 0..rows {
        let mut vec: Vec<f32> = Vec::new();
        let mut max_score = 0f32;
        let mut max_index = 0;

        for col in 0..cols {
            let value: f32 = *res.at_3d::<f32>(0, col, row)?; // (1 x M x 8400)
            if col > 3 {
                // the rest (after 4th) values are class scores
                if value > max_score {
                    max_score = value;
                    max_index = col - 4;
                }
            }
            vec.push(value);
        }
        if max_score > 0.25 {
            scores.push(max_score);
            class_index_list.push(max_index as i32);
            let cx = vec[0];
            let cy = vec[1];
            let w = vec[2];
            let h = vec[3];
            boxes.push(core::Rect {
                x: (((cx) - (w) / 2.0) * x_scale).round() as i32,
                y: (((cy) - (h) / 2.0) * y_scale).round() as i32,
                width: (w * x_scale).round() as i32,
                height: (h * y_scale).round() as i32,
            });
            indices.push(row as i32);
        }
    }
    dnn::nms_boxes(&boxes, &scores, 0.5, 0.5, &mut indices, 1.0, 0)?;
    let mut final_boxes: Vec<BoxDetection> = Vec::default();
    for i in &indices {
        let class = class_index_list.get(i as usize)?;
        let rect = boxes.get(i as usize)?;

        let bbox = BoxDetection {
            xmin: rect.x,
            ymin: rect.y,
            xmax: rect.x + rect.width,
            ymax: rect.y + rect.height,
            conf: scores.get(i as usize)?,
            class: class,
        };

        final_boxes.push(bbox);
    }

    for i in 0..final_boxes.len() {
        let bbox: &_ = &final_boxes[i];
        let rect = core::Rect::new(
            bbox.xmin,
            bbox.ymin,
            bbox.xmax - bbox.xmin,
            bbox.ymax - bbox.ymin,
        );

        let label = bbox.class.to_string();
        println!("{}", label);
        if label == "1" {
            let box_color = core::Scalar::from(((0.0, 255.0, 0.0))); // green color
            opencv::imgproc::rectangle(&mut umat, rect, box_color, 2, opencv::imgproc::LINE_8, 0)
                .unwrap();
        } else if label == "22" {
            let box_color = core::Scalar::new(0.0, 165.0, 255.0, 0.0); // orange color
            opencv::imgproc::rectangle(&mut umat, rect, box_color, 2, opencv::imgproc::LINE_8, 0)
                .unwrap();
        }
    }
    highgui::named_window("frame", highgui::WINDOW_OPENGL)?;
    highgui::imshow("frame", &umat)?;
    highgui::wait_key(0)?;
    

    Ok(())
}

pub struct BoxDetection {
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
    pub class: i32,
    pub conf: f32, // confidence score
}
