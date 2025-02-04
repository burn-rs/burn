#!/usr/bin/env python3

import torch
import torch.nn as nn
import torch.nn.functional as F
from safetensors.torch import save_file


class Model(nn.Module):
    def __init__(self):
        super(Model, self).__init__()
        self.conv1 = nn.Conv1d(2, 2, 2)
        self.conv2 = nn.Conv1d(2, 2, 2, bias=False)

    def forward(self, x):
        x = self.conv1(x)
        x = self.conv2(x)
        return x


def main():

    torch.set_printoptions(precision=8)
    torch.manual_seed(1)

    model = Model().to(torch.device("cpu"))

    save_file(model.state_dict(), "conv1d.safetensors")

    input = torch.rand(1, 2, 6)
    print("Input shape: {}", input.shape)
    print("Input: {}", input)
    output = model(input)
    print("Output: {}", output)
    print("Output Shape: {}", output.shape)


if __name__ == "__main__":
    main()
