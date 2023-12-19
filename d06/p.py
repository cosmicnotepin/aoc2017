import itertools

def main1(filename):
    mbs = [int(n) for n in open(filename, "r").read().strip().split()]
    seen = set((tuple(mbs),))
    print(f'mbs : {mbs }')
    print(f'seen : {seen }')

    old_len = 0
    while len(seen) > old_len:
        index_max = max(range(len(mbs)), key=mbs.__getitem__)
        to_dist = mbs[index_max]
        mbs[index_max] = 0
        for i in range(1, to_dist + 1):
            mbs[(index_max + i) % len(mbs)] += 1
        old_len = len(seen)
        seen.add((tuple(mbs),))

    print("p1: " + str(len(seen)))


def main2(filename):
    mbs = [int(n) for n in open(filename, "r").read().strip().split()]
    seen = {((tuple(mbs),)): 0}
    print(f'mbs : {mbs }')
    print(f'seen : {seen }')

    res = 0
    for rns in itertools.count():
        index_max = max(range(len(mbs)), key=mbs.__getitem__)
        to_dist = mbs[index_max]
        mbs[index_max] = 0
        for i in range(1, to_dist + 1):
            mbs[(index_max + i) % len(mbs)] += 1

        tpl = (tuple(mbs),)
        if tpl in seen:
            res = rns - seen[tpl]
            break
        else:
            seen[tpl] = rns

    print("p2: " + str(res))


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
    main2("input_t1")
    main2("input")
