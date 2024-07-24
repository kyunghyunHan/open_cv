mod affine;
mod binarization;
mod brihtness;
mod color_op;
mod contrast;
mod corners;
mod cv_dl;
mod deep_learning;
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
mod mat_op;
mod matching;
mod module;
mod mouse_event;
mod object_detection;
mod perspective;
mod qr_code;
mod resnet_image;
mod scanner;
mod snow;
mod ssd_face_detector;
mod storage;
mod torch;
mod tracker;
mod utils;
mod video;
mod web_cam_streaming_server;
mod window_capture;
mod yolo;
mod yolo_image;
pub fn main() {
    /*tracker */
    // tracker::main().unwrap();
    
    /* */
    // ssd_face_detector::main().unwrap();

    /*drawing */
    // drawing::main().unwrap();
    // mat_op::main().unwrap();

    /*google_net*/
    google_net::main().unwrap();
}
