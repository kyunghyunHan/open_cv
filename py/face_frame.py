import cv2
import pandas as pd

# 카메라 캡처 객체 생성
cap = cv2.VideoCapture(0)

# 얼굴 검출용 분류기 로드
face_cascade = cv2.CascadeClassifier(cv2.data.haarcascades + 'haarcascade_frontalface_default.xml')

# 데이터프레임 초기화
df = pd.DataFrame(columns=['Face ID', 'X', 'Y', 'Width', 'Height'])

face_id = 0

while True:
    # 카메라로부터 프레임 읽기
    ret, frame = cap.read()
    print(ret)
    # 프레임이 없으면 종료
    if not ret:
        break
    
    # 프레임을 흑백(그레이스케일)로 변환
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    
    # 얼굴 검출
    faces = face_cascade.detectMultiScale(gray_frame, scaleFactor=1.1, minNeighbors=5, minSize=(30, 30))
    
    # 검출된 얼굴 주변에 사각형 그리기 및 데이터프레임에 추가
    for (x, y, w, h) in faces:
        cv2.rectangle(frame, (x, y), (x+w, y+h), (0, 255, 0), 2)
        
        # 얼굴 영역을 이미지에서 잘라내어 저장
        face_img = frame[y:y+h, x:x+w]
        cv2.imshow('Face Image', face_img)
        print(face_img)
    
    # 결과 프레임 출력
    cv2.imshow('Face Detection', frame)
    
    # 'q' 키를 누르면 종료
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

# 사용한 자원 해제
cap.release()
cv2.destroyAllWindows()
