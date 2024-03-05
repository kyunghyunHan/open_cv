mod camera_init;
mod face_detection;
fn main(){
    face_detection::model::main().unwrap();
    // camera_init::model::main().unwrap();
}