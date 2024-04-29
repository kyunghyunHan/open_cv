use opencv::core::Point_;
use opencv::prelude::MatTraitConstManual;
use opencv::types::{VectorOfMat, VectorOfRect, VectorOfString,VectorOfi32,VectorOff32};
use opencv::{
    core::{self, Mat, MatTraitConst, Scalar, Size},
    dnn,
    dnn::{blob_from_image, read_net, NetTrait, NetTraitConst},
    highgui::{self, imshow, wait_key},
    imgcodecs::{self, imread},
    imgproc::{self, put_text, FONT_HERSHEY_SIMPLEX},
    Result,
};
use std::{env, fs::File, io::BufRead, io::BufReader};
pub fn main() -> Result<()> {
    // Load an image
    let args: Vec<String> = env::args().collect();
    let mut img = if args.len() < 2 {
        imread("./img/car.jpeg", imgcodecs::IMREAD_COLOR)?
    } else {
        imread(&args[1], imgcodecs::IMREAD_COLOR)?
    };

    if img.empty() {
        eprintln!("Image load failed!");
        return Ok(());
    }

    // Load networkbvlc_googlenet
    let mut net = read_net(
        "./dataset/bvlc_googlenet.caffemodel",
        "./dataset/deploy.prototxt",
        "",
    )?;

    if net.empty()? {
        eprintln!("Network load failed!");
        return Ok(());
    }

    // // Load class names
    let mut class_names = Vec::new();
    let file = File::open("./dataset/classification_classes_ILSVRC2012.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(name) = line {
            if !name.is_empty() {
                class_names.push(name);
            }
        }
    }

    // // Inference
    let input_blob = blob_from_image(
        &img,         
        1.0,
        Size::new(672, 672),
        Scalar::new(104.0, 117.0, 123.0, 0.0),
        false,
        false,
        core::CV_32F,
    )?;
    net.set_input(&input_blob, "data", 1.0, Scalar::default())?;

    // // Inference
    let input_blob = blob_from_image(
        &img,
        1.0,
        Size::new(224, 224),
        Scalar::new(104.0, 117.0, 123.0, 0.0),
        false,
        false,
        core::CV_32F,
    )?;
    let mut net_output = VectorOfMat::new();
    // let names = get_output_names(&net)?;
    let names = net.get_unconnected_out_layers_names()?;

    net.forward(&mut net_output, &names)?;

    let mut class_ids = VectorOfi32::new();
    let mut confidences =VectorOff32::new();
    let mut boxes = VectorOfRect::new();
    let img_width = img.cols();
    let img_height = img.rows();
    for (i, matrix) in net_output.iter().enumerate() {
        for j in 0..matrix.rows() {
            let data = matrix.at_row::<f32>(j as i32)?;
            // 먼저 첫 번째 Mat을 가져옵니다.
            let first_mat = net_output.get(i)?;

            // 첫 번째 행을 가져옵니다.
            let first_row = first_mat.row(j as i32)?;

            // 5번째 열부터 마지막 열까지의 범위를 가져옵니다.
            let range = core::Range::new(0 as i32, first_mat.cols())?;
            let conf_threshold = 0.;
            // scores를 가져옵니다.
            let scores = first_row.col_range(&range)?;
 
            let mut class_id_point = core::Point::default();
            let mut confidence = 0.5_f64;

            core::min_max_loc(
                &scores, 
                Some(&mut 0.), 
                Some(&mut confidence), 
                Some(&mut core::Point::new(0,0)), 
                Some(&mut class_id_point),
                &core::no_array()
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
        }
    }
    println!("{:?}", class_ids);
    println!("{:?}", confidences);
    println!("{:?}", boxes);
    let str = format!("{}", class_names[class_ids.get(0)? as usize]);

    put_text(
        &mut img,
        &str,
        Point_::from((10, 30)),
        FONT_HERSHEY_SIMPLEX,
        0.8,
        Scalar::from((0, 0, 255)),
        2,
        imgproc::LINE_AA,
        false,
    )?;
    imshow("img", &img)?;
    wait_key(0)?;

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
