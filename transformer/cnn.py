import torch 
import torch.nn as nn

pool = torch.nn.MaxPool2d(
    1,
    stride=None,
    padding=0,
    dilation=1
)


pool2 = torch.nn.AvgPool1d(
    kernel_size=1,
    stride=None,
    padding=0,
    count_include_pad=True
)
print(pool)
print(pool2)

class ConvolutionalNeuralNetwork(nn.Module):
    def __init__(self):
        super().__init__()


        self.conv1 = nn.Sequential(
            nn.Conv2d(
               in_channels=3,
               out_channels=16,
               kernel_size=3,
               stride=2,
               padding=1
            ),
             nn.ReLU(),
            nn.MaxPool2d(kernel_size=2,stride=2),
        ),
        self.conv2 = nn.Sequential(
            nn.Conv2d(
                    in_channels=16,out_channels=32,kernel_size=3,stride=1,padding=1
            ),
            nn.ReLU(),
            nn.MaxPool2d(kernel_size=2,stride=2),

        )
        self.fc = nn.Linear(32*32*32,10)
    
    def forward(self,x):
        x = self.conv1(x)
        x = self.conv2(x)
        x= torch.flatten(x)
        x = self.fc (x)
        return x


class SentenceClassifier(nn.Module):
    def __init__(self,pretrained_embedding,filter_sizes,max_length,dropout = 0.5):
        super().__init__()

        self.embedding = nn.Embedding.from_pretrained(
            torch.tensor(pretrained_embedding,dtype=torch.float32)
        )

        embedding_dim = self.embedding.weight.shape[1]

        conv=[]

        for size in filter_sizes:
            conv.append(
                nn.Sequential(
                    nn.Conv1d(
                        in_channels=embedding_dim,
                        out_channels=1,
                        kernel_size=size
                    ),
                    nn.ReLU(),
                    nn.MaxPool1d(kernel_size=max_length-size-1),
                )

            )
        self.conv_filters = nn.ModuleList(conv)
        output_size = len(filter_sizes)
        self.pre_classifier= nn.Linear(output_size,output_size)
        self.dropout = nn.Dropout(dropout)
        self.classifier= nn.Linear(output_size,1)
        
    def forward(self,inputs):
        embeddings= self.embedding(inputs)
        embeddings= embeddings.permute(0,2,1)

        conv_outputs= [conv(embeddings)for conv in self.conv_filters]
        concat_outputs= torch.cat([conv.squezze(-1) for conv in conv_outputs],dim =1)

        logits= self.pre_classifier(concat_outputs)
        logits = self.dropout(logits)
        logits = self.classifier(logits)
        return logits


    