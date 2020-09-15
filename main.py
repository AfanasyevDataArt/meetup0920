#!/usr/bin/python3
import random

def output_line():
    line = ""
    for i in range(0, 128):
        chank = random.randint(0, 0xFFFFFFFF)
        line += "{:08X}".format(chank)
    return line


if __name__ == '__main__':
    for j in range(0, 100000):
        print(output_line())

