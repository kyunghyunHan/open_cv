# open_cv

- opencv 에 대해 정리


## install
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

```
brew list --versions
brew -v update
brew upgrade --force --display-times
brew list --versions
brew -v install --force --display-times opencv"$BREW_OPENCV_VERSION"
brew link opencv"$BREW_OPENCV_VERSION"
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
## speed up
```
[profile.dev]
opt-level = 3
```



## Class
- Point_ :2차원 평면
- Size_:사각형 영역의 가로  세로 크기를 나타냄  ,(width,height)
- Rect_:사각형의 우치와 크기 정보를 표현,x,y는 사각형의 좌측 상단점좌표
- RotatedRect:회전된 사각형,center사각형의 중심 좌표.angle:회전 각도
- String:문자열
- Mat:2차원행렬뿐만 아니라 고차원 행렬 표현,한개이상의 채널을 가지며

```
s는 정수형 F는 부동 소수형
CV_8U 0
CV_8S 1
CV_16U 2
CV_16S 3
CV_32U 4
CV_32F 5
CV_65F 6
CV_16F 7
```


## mask

