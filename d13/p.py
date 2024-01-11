import itertools

def main1(filename):
    f = open(filename, "r")
    scanners = {}
    for line in f:
        depth, rg = line.strip().split(': ')
        scanners[int(depth)] = int(rg)

    severity = 0
    for depth, rg in scanners.items():
        if rg == 1:
            severity += depth * rg
            continue

        period = 2 * (rg - 2) + 2
        if depth % period == 0:
            severity += depth * rg

    print("p1: " + str(severity))


def ride(scanners, delay):
    for depth, rg in scanners.items():
        period = 2 * (rg - 2) + 2
        if (depth + delay) % period == 0:
            return False

    return True


def main2(filename):
    f = open(filename, "r")
    scanners = {}
    for line in f:
        depth, rg = line.strip().split(': ')
        scanners[int(depth)] = int(rg)

    for delay in itertools.count():
        if ride(scanners, delay):
            print("p2: " + str(delay))
            break

if __name__ == "__main__":
    main1("input_t1")
    main1("input")
    main2("input_t1")
    main2("input")
