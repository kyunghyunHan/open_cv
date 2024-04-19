import torch


# 간단한 예제 모델 정의
class SimpleModel(torch.nn.Module):
    def __init__(self):
        super(SimpleModel, self).__init__()
        self.fc = torch.nn.Linear(10, 2)

    def forward(self, x):
        return self.fc(x)

# 모델 인스턴스 생성
model = SimpleModel()

# 모델 가중치 초기화
torch.nn.init.xavier_uniform_(model.fc.weight)
torch.nn.init.constant_(model.fc.bias, 0)

# 예제 입력 데이터 생성
input_data = torch.randn(1, 10)

# 모델 추론
output = model(input_data)

# ONNX로 모델 저장
onnx_model_path = 'simple_model.onnx'
torch.onnx.export(model, input_data, onnx_model_path, export_params=True)




