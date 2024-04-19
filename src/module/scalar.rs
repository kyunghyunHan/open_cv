use opencv::{
    
    core::{self, InputArray, Scalar, ToInputArray, Vector, _InputArrayTraitConst, CV_8UC3},
    Result,
    boxed_ref::BoxedRef,
    
};
use opencv::core::_InputArray;
pub fn main() -> Result<()> {
    let gray = Scalar::from(128);
    /*Sclar */
    println!("{:?}", gray);

    let yello = Scalar::from((0, 255, 0, 0));

    let img1 = core::Mat::new_rows_cols_with_default(256, 256, CV_8UC3, yello)?;
    for i in 0..4 {
        println!("{}", yello[i]);
    }
    /*  input Array:입력인자 ,inputarray클래스의 인스터스 또는 변수 생성 제한 */
    let vec1:Vector<f64> = Vector::from_iter([1.2,3.4,-2.1].into_iter());
    let a= vec1.input_array()?;

    print_mat(a);
    Ok(())
}
fn print_mat(_mat: BoxedRef<_InputArray>){
      let mat = _mat.get_mat(-1).unwrap();
      println!("input_array:{:?}",mat);

     /*output_array:결과영상전달
      */
}