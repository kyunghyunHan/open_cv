import cv2
import dlib
import numpy as np

def main():
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

        # if len(faces) == 0:
        #     continue  # 얼굴이 감지되지 않았으면 다음 프레임으로 넘어감

        face = faces[0]

        # 얼굴 특징점 찾기
        landmarks = predictor(img, face)
        shape_2d = np.array([[p.x, p.y] for p in landmarks.parts()])

        # 특징점 처리
        for p in range(0, 68):
            cv2.circle(img, center=(landmarks.part(p).x, landmarks.part(p).y), radius=1, color=(255, 255, 255), thickness=2, lineType=cv2.LINE_AA)

        # 예시: shape_2d를 이용하여 특정 작업 수행
        # 여기에 원하는 작업을 수행하면 됩니다.
        # 예를 들어, 얼굴의 가로 길이 계산 등
        face_width = np.max(shape_2d[:, 0]) - np.min(shape_2d[:, 0])
        print("Face width:", face_width)

        cv2.imshow('img', img)
        cv2.waitKey(1)

if __name__ == "__main__":
    main()
