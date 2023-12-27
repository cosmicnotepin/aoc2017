def main1(filename):
    f = open(filename, "r")
    regs = {}
    insts = []
    for line in f:
        inst, cond = line.strip().split(' if ')
        reg, op, val = inst.split()
        exec('regs["' + reg + '"]' + '=0')
        inst = 'regs["' + reg + '"]' + op + val
        inst = inst.replace('dec', '-=')
        inst = inst.replace('inc', '+=')
        treg, cmp, cmpval = cond.split()
        cond = 'regs["' + treg + '"]' + cmp + cmpval
        insts.append((inst, cond))

    maximax = -99999999999999999
    for inst, cond in insts:
        if eval(cond):
            exec(inst)
        maximax = max(maximax, *(regs.values()))

    print("p1: " + str(max(regs.values())))
    print("p2: " + str(maximax))


def main2(filename):
    f = open(filename, "r")
    for line in f:
        pass

    print("p2: " + str())


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
    #main2("input_t2")
    #main2("input")
