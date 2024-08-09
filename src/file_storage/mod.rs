/*File Storage */
use opencv::{core::{FileStorage, FileStorageTrait, FileStorage_WRITE, Mat, Point, Vector}, highgui, Result};
pub fn main() -> Result<()> {
    write_data()?;
    Ok(())
}

fn write_data() -> Result<()> {
    let name= "Jane";
    let pt1 = Point::from((100,200));
    let scoress:Vector<i32> = Vector::from(vec![80,90,50]);
    let mat1 = Mat::default();

    let fs  = FileStorage::new(name, FileStorage_WRITE, "")?;
    fs.release();
    Ok(())
}
