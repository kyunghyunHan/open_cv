import cv2,dlib,sys
import numpy as np

scaler = 0.3

# initialize face detector and shape predictor
detector = dlib.get_frontal_face_detector()
predictor = dlib.shape_predictor('shape_predictor_68_face_landmarks.dat')



# load video
cap = cv2.VideoCapture('girl.mp4')
# load overlay image
# overlay = cv2.imread('ryan_transparent.png', cv2.IMREAD_UNCHANGED)


while True:
    ret, img = cap.read()
    if not ret:
        break
    cv2.imshow('img',img)
    cv2.waitKey(1)