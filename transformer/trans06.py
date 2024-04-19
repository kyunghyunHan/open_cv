import torch
import pandas as pd
from torch import nn
from torch import optim
from torch.utils.data import Dataset,DataLoader

class CustomDataset(Dataset):
    def __init__(self,file_path):
        df = pd.read_csv(file_path)
        self.x= df.iloc[:, 0].values
        self.y = df.iloc[:, 1].values
        self.length = len(df)
    
    def __getitem__(self, index):
        x = torch.FloatTensor([self.x[index] **2,self.x[index]])
        y= torch.FloatTensor([self.y[index]])
        return x,y
    
    def __len__(self):
        return self.length
    

class CustomModel(nn.Module):
    def __init__(self):
        super().__init__()
        self.layer = nn.Linear(2,1)

    def forward(self,x):
        x= self.layer(x)
        return x


device = "cuda" if torch.cuda.is_available() else "cpu"
model = CustomModel().to(device)

model_state_dict = torch.load("./model_state_dict.t7",map_location=device)
model.load_state_dict(model_state_dict)


with torch.no_grad():
    model.eval()
    inputs = torch.FloatTensor([
        [1 ** 2,1],
        [5 **2,5],
        [11 **2,11]
    ]).to(device)
    outputs =model(inputs)
    print(outputs)
