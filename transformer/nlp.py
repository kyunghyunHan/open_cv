

review= "현실과 구분 불가능한 cg. 시각적 즐거움은 최고! 더불어 ost는 더더욱 최고!!"
tokenized = review.split()
print(tokenized)
# 문자열 데이터는 split메서드르 이용하여 쉽게 토근화

review = "현실과 구분 불가능한 cg. 시각적 즐거움은 최고! 더불어 ost는 더더욱 최고!!"
tokenized= list(review)
print(tokenized)


from jamo import h2j,j2hcj
review = "현실과 구분 불가능한 cg. 시각적 즐거움은 최고! 더불어 ost는 더더욱 최고!!"
decomoposed = j2hcj(h2j(review))
tokenized = list(decomoposed)
print(tokenized)


#형태소 토근화
# 그 -는 나 -에게 같은 의미를 가지고있는

import nltk
from nltk import tokenize

nltk.download("punkt")
nltk.download("averaged_perceptron_tagger")

sentence= "Those who can imagine anything, can create the impossible."

word_tokens= tokenize.word_tokenize(sentence)
send_tokens= tokenize.sent_tokenize(sentence)

print(word_tokens)
print(send_tokens)

from nltk import tag

pos = tag.pos_tag(word_tokens)
print(pos)