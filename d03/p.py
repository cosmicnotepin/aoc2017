import cmath
from collections import defaultdict

def get_coords(target):
    coords = complex(0,0)
    nr = 1
    side_length = 0
    while True:
        for step in [complex(0,1), complex(-1, 0),
                complex(0, -1), complex(1,0)]:
            for j in range(side_length):
                coords += step
                nr += 1 
                if nr == target:
                    return coords
        coords += complex(1, -1)
        side_length += 2

def main1(target):
    coords = get_coords(target)
    print("p1: " + str(abs(coords.real) + abs(coords.imag)))


def main2(target):
    eightn = [complex(x, y) for x in (-1, 0, 1) for y in (-1, 0, 1)]
    coords = complex(0,0)
    spiral = defaultdict(int)
    nr = 1
    spiral[coords] = nr
    side_length = 0
    while True:
        for step in [complex(0, 1), complex(-1, 0),
                complex(0, -1), complex(1, 0)]:
            for j in range(side_length):
                coords += step
                nr = sum(spiral[coords + n] for n in eightn)
                spiral[coords] = nr
                if nr > target:
                    print(f'nr: {nr}')
                    return
        coords += complex(1, -1)
        side_length += 2


if __name__ == "__main__":
    #main1(1)
    main1(12)
    main1(23)
    main1(1024)
    main1(312051)
    main2(312051)
