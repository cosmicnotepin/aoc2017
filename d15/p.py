def main1(a_start, b_start):
    divider = 2147483647
    a_mul = 16807
    b_mul = 48271
    judge = 0xFFFF

    matches = 0
    for _ in range(40000000):
        a_start = (a_start * a_mul) % divider
        b_start = (b_start * b_mul) % divider
        matches += (a_start & judge) == (b_start & judge)


    print("p1: " + str(matches))


def main2(a_start, b_start):
    divider = 2147483647
    a_mul = 16807
    b_mul = 48271
    judge = 0xFFFF

    matches = 0
    tries = 0
    while tries < 5000000:
        while (a_start := (a_start * a_mul) % divider) % 4 != 0:
            pass
        while (b_start := (b_start * b_mul) % divider) % 8 != 0:
            pass
        matches += (a_start & judge) == (b_start & judge)
        tries += 1


    print("p2: " + str(matches))


if __name__ == "__main__":
    main1(65, 8921)
    main1(883, 879)
    main2(65, 8921)
    main2(883, 879)
