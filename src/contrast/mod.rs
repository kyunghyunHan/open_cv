use opencv::{
    core::{
        self, add, mul_f64_mat, multiply, no_array, subtract, Mat, MatExprTraitConst,
        MatTraitConst, Scalar, Scalar_,
    },
    highgui::{self, destroy_all_windows, imshow, wait_key},
    imgcodecs::{self, imread, IMREAD_GRAYSCALE},
    imgproc, Result,
};
/*영상의 명암비 조절

명암비란 영상에서 밝은 영역과 어두운 영역 사이에 드러나는 밝기 차이의 강도를 말하고 명암 대비 또는 contrast라고도 한다  영상이 전반적으로 어둡거나 또는 전반적으로 밝은 픽셀로만 구성된 경우 명암비가 낮다고 표현하며 반면애 밝은 영역과 어두운 영역이 골고루 섞여있는 영상은 명암비가 높다고 말한다
일반적으로 명암비가 낮은 영상은 객체간의 구분이 잘 되지 않아 전반적으로 흐릿하게 느껴지고 명암비가 높은 영상은 사물의 구분이 잘되며 선명한 느낌을 준다

일반적으로 명암비가 높은 사진이 잘찍은 사진처럼 보이기 떄문에 대부분의 디지털 카메라는 명암비가 높은 사진을 촬영하도록 설정되어 있다.
이미 촬영된 경우에는 픽셀의 밝기 값을 조절하여 전체적인 명암비를 높이거나 낮출수 있다.
명암비는 기본적으로 곱셈 연산을 하여 구현한다

수식을 보면 src는 입력영상 dst는 출력영상 그리고 상수 n은 0보다 큰 양의 실수이다.
입력영상 픽셀 값에 상수 s를 곱한 결과가 255가 커지는 경우가 발생할수 있으므로 포화연산도 함께 사용해야 한다산수 s가 1보다 작은 경우에는 명암비가 낮아지며 s가 1보다 큰 경우에는 명암비가 높아지는 효과가 있다.
에를 들어 s가2인 경우 원본 영상에서 발기가 60이엿다면 120으로 변경이 되게 된다 위 수식을 사용하게 되면 영상이 전반적으로 어두워 지거나 또는 결과 영상의 밝기가 너무 포화되는 단점이 잇다.

s=0.5 인 그래프의 경우 결과 영상의 픽셀이 가질수 있는 범위가 0부터 128 사이로 제한되기 때문에 전체적으로 어두워지며 명암비가 감소한다.
s=2인경우 입력영상에서 0부터 128까지의 픽셀은 0부터 255사이의 값을 가지기 떄문에 명암비가 높아지며 128이상의 값을 가지는 값들은 모두포화되어 255의 값을 가지게 된다.

밑은 기본적인 구현

이미지를 보면 픽셀값이 전체적으로 포화되어 흰색으로 나타내는 영역이 너무 많아져 사물의 윤곽구분이 어려워졋다


효과적으로 명암비를 구분하기 위해서는 밝은 픽셀은 더 밝게 어두운 픽셀은 더욱 어둡게 만들어야 구분이 잘될것입니다.이떄 픽셀값이 밝고 어둡다는 기준을 어떻게 설정할것인지가 명암비 조절 결과 영상의 품질 차이를 가져올수 있다.
그레이스케일 범위 중간인 128 을 기준으로 설정할수도있고 입력 영상의 평균 밝기 를 구하여 기준으로 삼을수도 잇습니다.간단하게 128을 기준으로 한다면 입력영상의 픽셀값이 128보다 크면 더욱 밝게만들고 128보다 작으면 픽셀값을 더 작게 만들면됩니다.
반대로 명암비를 감소시키려면 큰 입력 영상 픽셀값은 좀더 작게만들고 128보다 작은 픽셀값은 오히려 128에 가깝게 증가시키면됩니다.

수식을 보면  알파는 -1보다 큰 실수이며 이수식은 항상 (128,128)의 좌표를 지나가고 알파에의해 기울기가 변경되는 직선의 방정식입니다.하지만 이렇게 되면 영상의 픽셀값은 0보다 작거나 255보다 커지는 경우가 발생할수 있으므로 포화연산까지 함꼐 수행해야합니다.

수식을보면 알파=-0.5인경우와 알파=-1.0인 경우의 함수그래프를 나타냈습니다.첫번째 그래프는 -0.5인경우이며 명암비를 감소시키는 함수 그래프입니다.이경우 범위는 64부터 192로 한정됩니다.

*/

pub fn main() -> Result<()> {
    // contrast()?;
    contrast2()?;
    Ok(())
}
fn contrast() -> Result<()> {
    let src: opencv::prelude::Mat = imread("./img/lenna.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("{}", "image load fiiled");
    }
    let s = 2.;
    let dst = (s * &src).into_result()?.to_mat()?;

    imshow("src", &src)?;
    imshow("dst", &dst)?;

    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn contrast2() -> Result<()> {
    let src = imread("./img/lenna.bmp", IMREAD_GRAYSCALE)?;
    if src.empty() {
        panic!("{}", "image load fiiled");
    }
    let alpha = 1.;
    let sc = Scalar_::all(128.);
    let mut dst = Mat::default();
    let mut dst_sub = Mat::default();
    let mut dst_mul = Mat::default();
    subtract(&src, &sc, &mut dst_sub, &no_array(), -1)?;
    multiply(&dst_sub, &Scalar::all(alpha), &mut dst_mul, 1., -1)?;
    add(&src, &dst_sub, &mut dst, &core::no_array(), -1)?;

    imshow("src", &src)?;
    imshow("dst", &dst)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}
