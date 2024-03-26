import cv2, dlib, sys
import numpy as np

scaler = 0.3

cap= cv2.VideoCapture('girl.mp4')

while True:
    ret, img = cap.read()
    if not ret:
        break
    
    img =cv2.resize(img,(int(img.shape[1]* scaler),int(img.shape[0]*scaler)))
    ori = img.copy()

    cv2.imshow('img',img)
    cv2.waitKey(1)