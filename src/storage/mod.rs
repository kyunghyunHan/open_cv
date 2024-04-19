use opencv::{
    core::{FileStorage, Mat, Point_, Vector, CV_32F},
    Result};
/*Stoage
일반적인 Mat영상데이터는 imwrite함수 사용 저장
일반적인 행렬은 영상파일 형식으로 저장 불가능


*/
pub fn main() -> Result<()> {
    write_data()?;
    Ok(())
}
fn write_data() -> Result<()> {
    let name = "Jane";
    let age = 10;
    let pt1 = Point_::from((100, 200));
    let mut scores: Vector<i32> = Vector::from_iter([80, 90, 50]);
    // Define the data array
    let data = vec![1.0, 1.5, 2.0, 3.2];

    // Calculate the stride (number of bytes between successive rows)
    let stride = 2 * 4; // 2 columns * 4 bytes per element (CV_32F)

    // Create a new 2x2 Mat object with the data
    let mat1 = unsafe {
        Mat::new_rows_cols_with_data(2, 2, CV_32F, data.as_ptr() as *mut std::ffi::c_void, stride)
            .unwrap()
    };
    println!("{:?}", mat1);

    let fs = FileStorage::new_def("aa", 0)?;
    println!("{:?}",fs);
    Ok(())
}
