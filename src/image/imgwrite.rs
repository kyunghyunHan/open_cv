use opencv::{
    Result,
    core::{self, Mat, MatTraitConstManual, Vec3b, Vector, CV_8UC3},
    highgui::{self, imshow,},
    imgcodecs,
    imgproc,
    types,
};
use opencv::prelude::MatTraitConst;
pub fn main() -> Result<()> {
    let source_img = imgcodecs::imread("car.jpeg", imgcodecs::IMREAD_UNCHANGED)?;
    
    highgui::imshow("hello opencv!", &source_img)?;
    // let resized_img = resize_with_padding(&rgb_mat, [192, 192]);
    // let img_vec_3d = mat_to_vec_3d(&source_img);
    // println!("{:?}",img_vec_3d);
    // println!("{:?}",source_img.to_vec_2d::<u8>().unwrap());
    
    let test:Vec<Vec<Vec3b>>= source_img.to_vec_2d()?;

    println!("test{:?}",test);
    loop {
        let k = highgui::wait_key(0)?;
        if k == 27 {
            highgui::destroy_all_windows()?;
            break;
        } else if k as u8 as char == 's' {
            println!("Write");
            imgcodecs::imwrite("s.png", &source_img, &Vector::new())?;
        } else if k as u8 as char == 'a' {
            println!("End");
            highgui::destroy_all_windows()?;
            break;
        }
    }

    Ok(())
}
// fn mat_to_vec_3d(mat: &core::Mat) -> Vec<Vec<Vec<u8>>> {
//     let mut result = Vec::new();
//     for row_idx in 0..mat.rows() {
//         let mut row = Vec::new();
//         for col_idx in 0..mat.cols() {
//             let pixel = mat.at_2d::<core::Vec3b>(row_idx, col_idx).unwrap();
//             let pixel_vec = vec![pixel[0], pixel[1], pixel[2]]; // BGR 순서
//             row.push(pixel_vec);
//         }
//         result.push(row);
//     }
//     result
// }

