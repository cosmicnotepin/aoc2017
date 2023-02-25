from timeit import default_timer as timer
import sys

def main1(f):
    line = f.read()

    return 0


def main2(f):
    line = f.read()

    return 0


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
