import operator
from functools import reduce

def main1(length, filename):
    lens = list(map(int, open(filename, "r").readline().strip().split(',')))
    lst = list(range(length))
    cur_pos = 0
    skip = 0
    for l in lens:
        extr = []
        for i in range(l):
            extr.append(lst[(cur_pos + i) % length])
        for i in range(l):
            lst[(cur_pos + i) % length] = extr[l-1-i] 
        cur_pos += l + skip
        skip += 1

    print("p1: " + str(lst[0] * lst[1]))


def main2(length, filename):
    lens = list(map(ord, open(filename, "r").read().strip()))
    lens.extend([17, 31, 73, 47, 23])
    lst = list(range(length))
    cur_pos = 0
    skip = 0
    for _ in range(64):
        for l in lens:
            extr = []
            for i in range(l):
                extr.append(lst[(cur_pos + i) % length])
            for i in range(l):
                lst[(cur_pos + i) % length] = extr[l-1-i] 
            cur_pos += l + skip
            skip += 1

    dense = []
    for i in range(16):
        dense.append(reduce(operator.xor, lst[i*16:i*16+16]))

    hexhex = ''
    for e in dense:
        hexhex += f'{e:0=2x}'

    print(f'hexhex : {hexhex }')


if __name__ == "__main__":
    main1(5, "input_t1")
    main1(256, "input")
    main2(256, "input_t2")
    main2(256, "input")
