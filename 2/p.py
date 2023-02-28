from timeit import default_timer as timer
import sys
from itertools import combinations

def main1(f):
    res = 0
    for line in f:
        numbers = [int(n) for n in line.split()]
        res += max(numbers) - min(numbers)

    return res


def main2(f):
    res = 0
    def even_div(numbers):
        print(f'numbers: {numbers}')
        for n1, n2 in combinations(numbers, 2):
            print(f'n1, n2: {n1, n2}')
            if n1 % n2 == 0:
                return n1 // n2;
            if n2 % n1 == 0:
                return n2 // n1;

    for line in f:
        numbers = [int(n) for n in line.split()]
        res += even_div(numbers)

    return res


def main():
    funs = (main1, main2)
    start = timer()
    with open(sys.argv[2], "r") as f:
        res = funs[int(sys.argv[1])-1](f)
    end = timer()
    print('result: ' + str(res))
    print("time: " + f'{end - start:3.6f}')

if __name__ == "__main__":
    main()
