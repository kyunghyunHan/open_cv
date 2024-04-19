from transformers import GPT2LMHeadModel


model = GPT2LMHeadModel.from_pretrained(pretrained_model_name_or_path="gpt2")

for main_name,main_module in model.named_children():
    print(main_name)
    for sub_name,sub_module in main_module.named_children():
        print("L",sub_name)
        for ssub_name,ssub_module in sub_module.named_children():
            print("|  l",ssub_name)
            for sssb_name,sssb_module in ssub_module.named_children():
                print("|   |   L",sssb_name)





from transformers import pipeline

generator = pipeline(task="text-generation",model="gpt2")
outputs= generator(
    text_inputs= "Machine learning is",
    max_length =20,
    num_return_sequences=3,
    pad_token_id= generator.tokenizer.eos_token_id
)
print(outputs)