import cv2 as cv
import sys

img = cv.imread("./img/face.jpg")
if img is None:
    sys.exit("파일을 찾을수 없습니다")
cv.imshow("image Display",img)
cv.waitKey()
cv.destroyAllWindows()