import numpy as np
import torch
import torch.nn as nn
from torchvision.transforms import transforms
from torch.utils.data import DataLoader
from data_loader import eyes_dataset
from model import Net
import torch.optim as optim
def accuracy(y_pred, y_test):
    y_pred_tag = torch.round(torch.sigmoid(y_pred))

    correct_results_sum = (y_pred_tag == y_test).sum().float()
    acc = correct_results_sum / y_test.shape[0]
    acc = torch.round(acc * 100)

    return acc
def train_model(x_train, y_train, model, save_path, epochs=50, batch_size=32, learning_rate=0.0001):
    # Define transformation
    train_transform = transforms.Compose([
        transforms.ToTensor(),
        transforms.RandomRotation(10),
        transforms.RandomHorizontalFlip(),
    ])

    # Create dataset and dataloader
    train_dataset = eyes_dataset(x_train, y_train, transform=train_transform)
    train_dataloader = DataLoader(train_dataset, batch_size=batch_size, shuffle=True, num_workers=4)

    # Move model to device
    device = torch.device("mps")
    model.to(device)

    # Define loss function and optimizer
    criterion = nn.BCEWithLogitsLoss()
    optimizer = optim.Adam(model.parameters(), lr=learning_rate)

    # Training loop
    for epoch in range(epochs):
        print(f"Epoch {epoch + 1}/{epochs}")
        running_loss = 0.0
        running_acc = 0.0

        model.train()

        for i, data in enumerate(train_dataloader, 0):
            inputs, labels = data[0].to(device), data[1].to(device)

            inputs = inputs.transpose(1, 3).transpose(2, 3)

            optimizer.zero_grad()

            outputs = model(inputs)

            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()

            running_loss += loss.item()
            running_acc += accuracy(outputs, labels)

            if i % 80 == 79:
                print('Mini-batch [%d/%d] - Loss: %.5f - Accuracy: %.5f' % (
                    i + 1, len(train_dataloader), running_loss / 80, running_acc / 80))
                running_loss = 0.0

    print("Training finished.")
    # Save trained model
    torch.save(model.state_dict(), save_path)

# Load data
x_train = np.load('./dataset/x_train.npy').astype(np.float32)  # (2586, 26, 34, 1)
y_train = np.load('./dataset/y_train.npy').astype(np.float32)  # (2586, 1)

# Initialize model
model = Net()

# Train model

if __name__ == '__main__':
    train_model(x_train, y_train, model, 'weights/trained.pth')
