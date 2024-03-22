import torch
from torch import nn
import torch.nn.functional as F
 
class Net(nn.Module):
    def __init__(self):
        super().__init__()
        self.lin1 = nn.Linear(5,5)
 
    def forward(self,x):
        net = self.lin1(x)
        return net
 
device = torch.device("mps") if torch.backends.mps.is_available() else "cpu"
print(f"device: {device}")
 
# MPS 장치에 바로 tensor를 생성합니다.
x = torch.ones(5, device=device)
 
# GPU 상에서 연산 진행
y = x * 2
 
# 또는, 다른 장치와 마찬가지로 MPS로 이동할 수도 있습니다.
model = Net()# 어떤 모델의 객체를 생성한 뒤,
model.to(device) # MPS 장치로 이동합니다.
 
# 이제 모델과 텐서를 호출하면 GPU에서 연산이 이뤄집니다.
pred = model(x)
print(pred)


a = torch.tensor([1, 2, 3, 4]).to("mps")
print(a)