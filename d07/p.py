import functools
from collections import defaultdict

def main1(filename):
    f = open(filename, "r")
    prgms = {}
    for line in f:
        if '-' in line:
            prgm, chldn = line.strip().split(' -> ')
            name, weight = prgm.split()
            chldn = chldn.split(', ')
        else:
            name, weight = line.strip().split()
            chldn = []
        prgms[name] = [int(weight[1:-1]), chldn]
    for name in prgms:
        if not any(name in chldn for _, chldn in prgms.values()):
            break

    print("p1: " + str(name))

    @functools.cache
    def weight_rec(name):
        return prgms[name][0] + sum(weight_rec(chld) for chld in prgms[name][1])

    def mark_bal_rec(name):
        weights = set()
        for chld in prgms[name][1]:
            weights.add(weight_rec(chld))
            mark_bal_rec(chld)
        prgms[name].append(len(weights) < 2)

    mark_bal_rec(name)

    for name in prgms:
        if not prgms[name][2] and all(prgms[n][2] for n in prgms[name][1]):
            break

    weights = defaultdict(list)
    for chld in prgms[name][1]:
        weights[weight_rec(chld)].append(chld)
    for key, value in weights.items():
        if len(value) == 1:
            break
    for key2 in weights:
        if key != key2:
            diff = key - key2
            break
    res = prgms[value[0]][0] - diff

    print("p2: " + str(res))

if __name__ == "__main__":
    main1("input_t1")
    main1("input")
