mod affine;
mod binarization;
mod brihtness;
mod color_op;
mod contrast;
mod corners;
mod cv_dl;
mod drawing;
mod edges;
mod filter;
mod google_net;
mod histogram;
mod hog;
mod hough;
mod image;
mod keyboard;
mod keypoint;
mod labeling;
mod machine_learning;
mod matching;
mod module;
mod mouse_event;
mod object_detection;
mod perspective;
mod qr_code;
mod snow;
mod ssd_face_detector;
mod storage;
mod torch;
mod utils;
mod video;
mod deep_learning;
mod yolo;

pub fn main() {
    yolo::main().unwrap()
    // object_detection::main().unwrap();
    //test
}
