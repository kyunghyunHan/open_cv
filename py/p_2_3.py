import cv2 as cv
import sys

img = cv.imread("./img/face.jpg")
if img is None:
    sys.exit("파일을 찾을수 없습니다")

gray= cv.cvtColor(img,cv.COLOR_BGR2GRAY) # BGB컬러 영상을 명암 영상으로 변환
gray_small= cv.resize(gray,dsize=(0,0),fx=0.5,fy=0.5) # 반으로 축소
cv.imshow("image Display",gray)
cv.imshow("image Display2",gray_small)
cv.waitKey()
cv.destroyAllWindows()