def main1(filename):
    f = open(filename, "r")
    jos = [int(o) for o in f.read().strip().split('\n')]
    cur_pos = 0
    res = 0
    while cur_pos >= 0 and cur_pos < len(jos):
        o = jos[cur_pos]
        jos[cur_pos] += 1
        cur_pos += o
        res += 1

    print("p1: " + str(res))


def main2(filename):
    f = open(filename, "r")
    jos = [int(o) for o in f.read().strip().split('\n')]
    cur_pos = 0
    res = 0
    while cur_pos >= 0 and cur_pos < len(jos):
        o = jos[cur_pos]
        if o >= 3:
            jos[cur_pos] -= 1
        else:
            jos[cur_pos] += 1
        cur_pos += o
        res += 1

    print("p2: " + str(res))


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
    main2("input_t1")
    main2("input")
