import math
import torch
from torch import nn
from matplotlib import pyplot as plt


class PositionalEncoding(nn.Module):
    def __init__(self,d_model,max_len,dropout=0.1):
        super().__init__()
        self.dropout  = nn.Dropout(p= dropout)


        position = torch.arange(max_len).unsqueeze(1)
        div_term = torch.exp(
            torch.arange(0,d_model,2) * (-math.log(10000.0)/d_model)
        )
        pe = torch.zeros(max_len,1,d_model)
        pe[:,0,0::2] = torch.sin(position * div_term)
        pe[:,0,1::2] = torch.cos(position * div_term)
        self.register_buffer("pe",pe)

    def forward(self,x):
        x= x+self.pe[: x.size(0)]
        return self.dropout(x)
    
encoding = PositionalEncoding(d_model =128,max_len=50)

plt.pcolormesh(encoding.pe.numpy().squeeze(),cmap="RdBu")
plt.xlabel("Embedding Dimensiuon")
plt.xlim((0,128))
plt.ylabel("Position")
plt.colorbar()
plt.show()


