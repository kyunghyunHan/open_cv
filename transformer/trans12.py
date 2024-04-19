#배치 정규화
import torch
from torch import nn

x = torch.FloatTensor(
    [
        [-0.6577,-0.5797,0.6360],
        [0.7392,0.2145,1.523],
        [0.2432,0.5662,0.322]
    ]
)

print(nn.BatchNorm1d(3)(x))

#배치 정규화 클레스
# m= torch.nn.BatchNorm1d(
#     num_features,
#     eps=1e-05
# )