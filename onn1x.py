import torch
from ultralytics import YOLO

model = YOLO('./best.pt')
model.export(format="onnx")

# # 간단한 예제 모델 정의
# class SimpleModel(torch.nn.Module):
#     def __init__(self):
#         super(SimpleModel, self).__init__()
#         self.fc = torch.nn.Linear(10, 2)

#     def forward(self, x):
#         return self.fc(x)

# # 모델 인스턴스 생성
# model = SimpleModel()

# # 모델 가중치 초기화
# torch.nn.init.xavier_uniform_(model.fc.weight)
# torch.nn.init.constant_(model.fc.bias, 0)

# # 예제 입력 데이터 생성
# input_data = torch.randn(1, 10)

# # 모델 추론
# output = model(input_data)

# # ONNX로 모델 저장
# onnx_model_path = 'simple_model.onnx'
# torch.onnx.export(model, input_data, onnx_model_path, export_params=True)



# 모델에 대한 입력값
# x = torch.randn(batch_size, 1, 224, 224, requires_grad=True)
# torch_out = torch_model(x)

# # 모델 변환
# torch.onnx.export(torch_model,               # 실행될 모델
#                   x,                         # 모델 입력값 (튜플 또는 여러 입력값들도 가능)
#                   "super_resolution.onnx",   # 모델 저장 경로 (파일 또는 파일과 유사한 객체 모두 가능)
#                   export_params=True,        # 모델 파일 안에 학습된 모델 가중치를 저장할지의 여부
#                   opset_version=10,          # 모델을 변환할 때 사용할 ONNX 버전
#                   do_constant_folding=True,  # 최적화시 상수폴딩을 사용할지의 여부
#                   input_names = ['input'],   # 모델의 입력값을 가리키는 이름
#                   output_names = ['output'], # 모델의 출력값을 가리키는 이름
#                   dynamic_axes={'input' : {0 : 'batch_size'},    # 가변적인 길이를 가진 차원
#                                 'output' : {0 : 'batch_size'}})
