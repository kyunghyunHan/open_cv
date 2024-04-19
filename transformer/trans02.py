import numpy as np

x= np.array(
    [[1],[2],[3],[4],[5],[6],[7],[8],[9],[10],
     [11],[12],[13],[14],[15],[16],[17],[18],[19],[20],
     [21],[22],[23],[24],[25],[26],[27],[28],[29],[30]]
)
y= np.array(
    [[0.94],[1.98],[2.88],[3.92],[3.96],[4.55],[5.64],[6.3],[7.44],[9.1],
     [8.46],[9.5],[10.67],[11.16],[14],[11.83],[14.4],[14.25],[16.2],[16.32],
     [17.46],[19.8],[18],[21.34],[22],[22.5],[24.57],[26.04],[21.6],[28.8]]
)
weight= 0.0
bias = 0.0
learning_rate= 0.005
for epoch in range(10000):
    y_hat = weight * x +bias
    cost = ((y - y_hat)**2).mean()

    weight  = weight - learning_rate * ((y_hat-y)*x).mean()
    bias = bias - learning_rate * (y_hat -y).mean()
    if (epoch +1 ) % 1000==0:
        print(f"EPOCH:{epoch+1:4d}, weight: {weight:.3f},Bias:{bias:.3f},cost:{cost:.3f}")