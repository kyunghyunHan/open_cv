from PIL import Image
from torchvision import transforms
import matplotlib.pyplot as plt

transform = transforms.Compose([
    # transforms.Resize(size=(512, 512)),
    transforms.RandomRotation(degrees=30, expand=False, center=None),
    transforms.RandomHorizontalFlip(p=0.5),
    transforms.RandomVerticalFlip(p=0.5),
    transforms.ToTensor()
])

image = Image.open("./dataset/cat.jpg")
transformed_image = transform(image)

print("Transformed Image Shape:", transformed_image.shape)

# Tensor를 이미지로 변환하기 위해 변환된 이미지를 0과 1 사이의 값으로 스케일링합니다.
transformed_image = transformed_image.permute(1, 2, 0)  # 채널 순서를 변경합니다.
transformed_image = transformed_image.numpy()  # Tensor를 NumPy 배열로 변환합니다.
plt.imshow(transformed_image)
plt.axis('off')  # 축 제거
plt.show()
