import cv2
import dlib
import numpy as np
scaler = 0.3
detector = dlib.get_frontal_face_detector()
predictor = dlib.shape_predictor('./dataset/shape_predictor_68_face_landmarks.dat')
cap = cv2.VideoCapture('girl.mp4')

while True:
    ret, img = cap.read()
    if not ret:
        break

    img = cv2.resize(img, (int(img.shape[1] * scaler), int(img.shape[0] * scaler)))
    ori = img.copy()
    faces = detector(img)

    if len(faces) == 0:
        continue  # 얼굴이 감지되지 않았으면 다음 프레임으로 넘어감

    face = faces[0]

    # 얼굴 특징점 찾기
    landmarks = predictor(img, face)
    # shape_2d = np.array([[p.x, p.y] for p in landmarks.parts()])
    # 특징점 처리
    for p in range(0, landmarks.num_parts):
        cv2.circle(img, center=(landmarks.part(p).x, landmarks.part(p).y), radius=1, color=(255, 255, 255), thickness=2, lineType=cv2.LINE_AA)

    cv2.imshow('img', img)
    cv2.waitKey(1)
