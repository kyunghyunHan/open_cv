# 체크포인트 불러오기
import torch
import pandas as pd
from torch import nn
from torch import optim
from torch.utils.data import Dataset,DataLoader,random_split

class CustomDataset(Dataset):
    def __init__(self,file_path):
        df = pd.read_csv(file_path)
        self.x1= df.iloc[:, 0].values
        self.x2 = df.iloc[:, 1].values
        self.x3 = df.iloc[:, 2].values
        self.y = df.iloc[:, 3].values
        self.length = len(df)
    
    def __getitem__(self, index):
        x = torch.FloatTensor([self.x1[index],self.x2[index],self.x3[index]])
        y= torch.FloatTensor([int(self.y[index])])
        return x,y
    
    def __len__(self):
        return self.length
    

class CustomModel(nn.Module):
    def __init__(self):
        super().__init__()
        self.layer = nn.Sequential(
            nn.Linear(3,1),
            nn.Sigmoid()
        )

    def forward(self,x):
        x= self.layer(x)
        return x
    
dataset= CustomDataset("./dataset/binary.csv")
dataset_size= len(dataset)
train_size= int(dataset_size * 0.8)
validation_size= int(dataset_size *0.1 )
test_size= dataset_size- train_size-validation_size

train_dataset, validation_dataset,test_dataset = random_split(
   dataset,[train_size,validation_size,test_size],torch.manual_seed(4)
)
train_dataloader= DataLoader(train_dataset,batch_size=64,shuffle=True,drop_last=True)


validation_dataloader = DataLoader(validation_dataset,batch_size=4,shuffle=True,drop_last=True)

test_dataloader= DataLoader(test_dataset,batch_size=4,shuffle=True,drop_last=True)

device = "cuda" if torch.cuda.is_available() else "cpu"
model = CustomModel().to(device)
criterion = nn.BCELoss().to(device)
optimizer = optim.SGD(model.parameters(),lr=0.0001)
# checkpoint = torch.load("./checkpoint/checkpoint-6.pt")
# model.load_state_dict(checkpoint["model_state_dict"])
# optimizer.load_state_dict(checkpoint["optimizer_state_dict"])
# checkpoint_epoch = checkpoint["epoch"]
# checkpoint_description = checkpoint["description"]
# print(checkpoint_description)
for epoch in range(10000):
    cost =0.0
    for x,y in train_dataloader:
        x= x.to(device)
        y= y.to(device)

        output =  model (x)
        loss = criterion(output,y)

        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        

        cost+=loss
    cost= cost/len(train_dataloader)


    if (epoch +1) % 1000==0:
        print(f"EPOCH:{epoch+1:4d},model:{list(model.parameters())},cost:{cost:.3f}")

with torch.no_grad():
    model.eval()
    for x,y in validation_dataloader:
        x= x.to(device)
        y= y.to(device)

        outputs= model(x)

        

        print(outputs)
        print(output>=torch.FloatTensor([0.5]).to(device))


# torch.save(
#     model,
#     "./model.t7"
# )
# torch.save(
#     model.state_dict(),
#     "./model_state_dict.t7"
# )