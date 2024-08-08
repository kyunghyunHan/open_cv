use opencv::objdetect::CascadeClassifier;

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
mod face_mark;
mod filter;
mod google_net;
mod histogram;
mod hog;
mod hough;
mod image;
mod image_gif;
mod keyboard;
mod keypoint;
mod labeling;
mod machine_learning;
mod mat_op;
mod matching;
mod module;
mod mouse_event;
mod object_detection;
mod odometry_algorithm;
mod perspective;
mod qr_code;
mod resnet_image;
mod scanner;
mod storage;
mod torch;
mod tracker;
mod utils;
mod video;
mod video_features;
mod warping;
mod web_cam_streaming_server;
mod yolo;
mod yolo_image;
pub fn main() {


    /*Mat OP */

    // mat_op::main().unwrap();

    /*Video Capture */
    video::main().unwrap();
    /*tracker */
    // tracker::main().unwrap();

    /* */
    // ssd_face_detector::main().unwrap();

    /*drawing */
    // drawing::main().unwrap();
    // mat_op::main().unwrap();

    /*google_net*/
    // google_net::main().unwrap();

    /*face_mark  */
    // face_mark::main().unwrap();

    /*video features */
    // video_features::main().unwrap()
    // ssd_face_detector::main().unwrap();
    // face_mark::main().unwrap();

    /*Warping */
    // warping::main().unwrap();

    // image_gif::main().unwrap();

    
}
