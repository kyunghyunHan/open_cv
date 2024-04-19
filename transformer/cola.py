import torch 
from torchtext.datasets import CoLA
from transformers import AutoTokenizer
from torch.utils.data import DataLoader

def collator(batch,tokenizer,device):
    source,labels,texts = zip(*batch)
    tokenizer = tokenizer(
        texts,
        padding= "longest",
        truncation = True,
        return_tensors= "pt"
    )
    input_ids = tokenizer["input_ids"].to(device)
    attention_mask = tokenizer["attention_mask"].to(device)
    labels = torch.tensor (labels,dtype=torch.long).to(device)
    return input_ids,attention_mask,labels


train_data = list(CoLA(split="train"))
valid_data = list(CoLA(split="dev"))
test_data = list(CoLA(split="test"))

# tokenizer = AutoTokenizer.from_pretrained("gpt2")
# tokenizer.pad_token  = tokenizer.eos_token

# epoch =3
# batch_size = 16
# device = "cuda" if torch.cuda.is_available() else "cpu"

# train_dataloader = DataLoader(
#     train_data,
#     batch_size=batch_size,
#     collate_fn=lambda x: collator(x,tokenizer,device),
#     shuffle=True
# )

# valid_dataloader = DataLoader(
#     valid_data,batch_size,collate_fn=lambda x:collator(x,tokenizer,device)
# )

# test_dataloader= DataLoader(
#     test_data,batch_size=batch_size,collate_fn=lambda x:collator(x,tokenizer,device)

# )



# print("Train Dataset Length : ",len(train_data))
# print("Valid Dataset Length : ",len(valid_data))
# print("Test Dataset Length : ",len(test_data))