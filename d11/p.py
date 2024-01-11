import numpy as np

compass2qrs = { 'n' : np.array([0,-1,1]),
                'ne': np.array([1,-1,0]),
                'se': np.array([1,0,-1]),
                's': np.array([0,1,-1]),
                'sw': np.array([-1,1,0]),
                'nw': np.array([-1,0,1]) }


def hex_dist(a,b):
    return np.sum(np.abs(b-a))//2


def main1(filename):
    steps = open(filename, "r").read().strip().split(',')
    start = np.array([0, 0, 0])
    cur = np.copy(start)
    max_dist = 0
    for step in steps:
        cur += compass2qrs[step]
        max_dist = max(max_dist, hex_dist(start, cur))

    print("p1: " + str(hex_dist(start, cur)))
    print("p2: " + str(max_dist))


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
