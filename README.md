# open_cv
## 
- m1기준

```
opencv = "0.90.0"
```
```
vi ~/.zshrc
```
```
brew install pkg-config
brew install cmake
brew install --debug llvm-dev
brew install libopencv-dev

```
```
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export PATH="/usr/local/opt/llvm/bin:$PATH"

```
## 데이터셋

- 사람얼굴감지 :haarcascade_frontalface_alt.xml
- 사람 눈감지 : haarcascades/haarcascade_eye_tree_eyeglasses.xml
- 고양이 얼굴감지 :haarcascades/haarcascade_frontalcatface.xml

 ## 주요모듈
 - calib3d : 카메라 켈브레이션과 3차원 재구성
 - 해열ㄹ,백터 등 opencv핵심 클래스와 연산함수
 - dnn:심층 신경망 기능
 - features2d:2차원 특징추출과  특징벡터기술 ,매칭방법
 - flann:다차원 공간에서 빠른 최근방 이웃 검색
 - highgui:영상의 화면출력,마우스 이벤트 처리등 사용자 인터페이스
 - imgcodecs:영상 파일 입출력
 - imgproc:필터링,기하하적적변환,색 공간 변환 등 영상처리 기능
 - ml:통게적 분류,회기 등 머신러닝 알고리즘
 - object:얼굴 ,보행자,검출등 객체검출
 - prhto:잡음제거
- 

## 

```
[profile.dev]
opt-level = 3

```