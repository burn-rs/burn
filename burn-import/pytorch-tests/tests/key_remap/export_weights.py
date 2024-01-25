#!/usr/bin/env python3

import torch
import torch.nn as nn
import torch.nn.functional as F

class ConvModule(nn.Module):
    def __init__(self):
        super(ConvModule, self).__init__()
        self.conv1 = nn.Conv2d(2, 2, (2,2))
        self.conv2 = nn.Conv2d(2, 2, (2,2), bias=False)
        
    def forward(self, x):
        x = self.conv1(x)
        x = self.conv2(x)
        return x

class Model(nn.Module):
    def __init__(self):
        super(Model, self).__init__()
        self.conv = ConvModule()
    
    def forward(self, x):
        x = self.conv(x)
        return x
    

def main():

    torch.set_printoptions(precision=8)
    torch.manual_seed(1)

    model = Model().to(torch.device("cpu"))

    torch.save(model.state_dict(), "key_remap.pt")
    
    input = torch.rand(1, 2, 5, 5)
    print("Input shape: {}", input.shape)
    print("Input: {}", input)
    output = model(input)
    print("Output: {}", output)
    print("Output Shape: {}", output.shape)

    


if __name__ == '__main__':
    main()
