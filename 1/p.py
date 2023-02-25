from timeit import default_timer as timer
import sys

def main1(f):
    line = f.read().strip('\n')
    return sum(int(x) for x,y in zip(line, line[1:] + line[0:1]) if x == y)


def main2(f):
    l = f.read().strip('\n')
    return sum(int(l[x]) for x in range(len(l)) if l[x] == l[(x+(len(l)//2))% len(l)])


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
