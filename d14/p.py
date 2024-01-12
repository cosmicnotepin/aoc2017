from functools import reduce
import operator

def knot_hash(key):
    lens = list(map(ord, key))
    lens.extend([17, 31, 73, 47, 23])
    length = 256
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

    return hexhex

def ff(start, grid, groups_count):
    neigh_offsets = [complex(0,1), complex(1, 0),
            complex(0, -1), complex(-1, 0)]
    todo = [start]
    while todo:
        cur = todo.pop(0)
        grid[cur] = groups_count
        for neigh_offset in neigh_offsets:
            neigh = cur + neigh_offset
            if grid.get(neigh, 0) == 1:
                todo.append(neigh)


def main1(keystring):
    total = 0
    grid = {}
    for i in range(128):
        row_hash = knot_hash(keystring + '-' + str(i))
        row = f'{int(row_hash, 16):0>128b}'
        for col, used in enumerate(row):
            if used == '1':
                grid[complex(i,col)] = 1
        total += row.count('1')

    print("p1: " + str(total))

    groups_count = 1
    for row in range(128):
        for col in range(128):
            if grid.get(complex(row,col), 0) == 1:
                groups_count += 1
                ff(complex(row,col), grid, groups_count)

    print("p2: " + str(groups_count-1))


if __name__ == "__main__":
    main1('flqrgnkx')
    main1('xlqgujun')
