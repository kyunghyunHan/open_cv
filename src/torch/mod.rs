use opencv::{highgui, prelude::*, videoio, Result,dnn};
pub fn main() -> Result<()> {
    let mut net = dnn::read_net("./simple_model.onnx", "", "")?;
	println!("{:?}",net);
	Ok(())
}