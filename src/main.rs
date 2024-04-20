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
mod hough;
mod image;
mod keyboard;
mod labeling;
mod module;
mod mouse_event;
mod perspective;
mod snow;
mod ssd_face_detector;
mod storage;
mod torch;
mod utils;
mod video;
mod object_detection;
pub fn main() {
    image::imgshow::main().unwrap();
    // object_detection::main().unwrap();
    //test
}
