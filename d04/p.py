def main1(filename):
    f = open(filename, "r")
    res = 0
    for line in f:
        words = line.strip().split()
        res += len(words) == len(set(words))

    print("p1: " + str(res))


def check_phrase(words):
    for i in range(len(words) - 1):
        for j in range(i+1, len(words)):
            if sorted(words[i]) == sorted(words[j]):
                return False
    return True

def main2(filename):
    f = open(filename, "r")
    res = 0
    for line in f:
        words = line.strip().split()
        res += check_phrase(words)

    print("p2: " + str(res))


if __name__ == "__main__":
    main1("input")
    main2("input")
