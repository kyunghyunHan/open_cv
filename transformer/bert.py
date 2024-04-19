import numpy as np
from datasets import load_dataset


news = load_dataset("argilla/news-summary",split="test")
df = news.to_pandas().sample(5000,random_state=42)[["text","prediction"]]
df["prediction"] = df["prediction"].map(lambda x: x[0]["text"])
train,valid,test = np.split(
    df.sample(frac=1,random_state=42),[int(0.6 * len(df)),int(0.8*len(df))]
)

print(f"Source News :{train.text.iloc[0][:200]}")
print(f"Sumarization : {train.prediction.iloc[0][:50]}")
print(f"Traing Data Size: {len(train)}")
print(f"Traing Data Size: {len(valid)}")
print(f"Testing Data Size : {len(test)}")
