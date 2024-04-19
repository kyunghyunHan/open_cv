# 가중치 초기화

from torch import nn

# class Net(nn.modules):
#     def __init__(self):
#         super().__init__()
#         self.layer= nn.Sequential(
#             nn.Linear(1,2),
#             nn.Sigmoid()
#         )
#         self.fc= nn.Linear(2,1)
#         self.__init_weights()


#     def __init_weights(self):
#         nn.init.xavier_uniform_(self.layer[0].weight)
#         self.layer[0].bias.data.fill_(0.01)

#         nn.init.xavier_uniform_(self.fc.weight)
#         self.fc.bias.data.fill_(0.01)

# model =Net()


## 가중치 초기화 함수 2

class Net(nn.Module):
    def __init__(self):
        super().__init__()
        self.layer= nn.Sequential(
            nn.Linear(1,2),
            nn.Sigmoid()
        )
        self.fc= nn.Linear(2,1)
        self.apply(self.__init_weights)


    def __init_weights(self,module):
        if isinstance(module,nn.Linear):
            nn.init.xavier_uniform_(module.weight)
            nn.init.constant_(module.bias,0.01)
        print(f"Apply:{module}")


model = Net()