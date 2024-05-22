use opencv::videoio::{VideoCaptureTrait, VideoCaptureTraitConst};
use opencv::{core, imgcodecs, videoio, Result};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, cam: Arc<Mutex<videoio::VideoCapture>>) -> Result<()> {
    let mut frame = core::Mat::default();
    let mut buf: core::Vector<u8> = core::Vector::new();

    {
        let mut cam = cam.lock().unwrap();
        cam.read(&mut frame)?;
    }

    buf.clear();
    imgcodecs::imencode(".jpg", &frame, &mut buf, &core::Vector::new())?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n"
    );
    stream.write_all(response.as_bytes()).unwrap();

    loop {
        {
            let mut cam = cam.lock().unwrap();
            cam.read(&mut frame)?;
        }
        buf.clear();
        let _ = imgcodecs::imencode(".jpg", &frame, &mut buf, &core::Vector::new());

        let image_data = format!(
            "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
            buf.len()
        );

        stream.write_all(image_data.as_bytes()).unwrap();
        stream.write_all(buf.as_slice()).unwrap();
        stream.write_all(b"\r\n").unwrap();
        stream.flush().unwrap();
    }
}

pub fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Server listening on port 3000");

    // Create a video capture object
    let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    if !cam.is_opened()? {
        panic!("Unable to open camera");
    }

    let cam = Arc::new(Mutex::new(cam));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cam_clone = Arc::clone(&cam);
                thread::spawn(move || {
                    handle_client(stream, cam_clone).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
