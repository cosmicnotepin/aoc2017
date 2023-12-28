import re

def main1(filename):
    f = open(filename, "r")
    total_score = 0
    total_garbage_count = 0
    for line in f:
        line = re.sub(r'!.', '', line.strip())
        garbage_count = 0
        subs = 1
        while subs != 0:
            next_line, subs = re.subn(r'<.*?>', '', line)
            garbage_count += max(0, len(line) - len(next_line) - 2 * subs)
            line = next_line

        line = re.sub(r',', '', line)
        score = 0
        level = 0
        for c in line:
            if c == '{':
                level += 1
            else:
                score += level
                level -= 1

        total_score += score
        total_garbage_count = garbage_count

    print("p1: " + str(total_score))
    print("p2: " + str(garbage_count))


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
