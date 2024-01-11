def ff(conns, start):
    todo = [start]
    visited = set([start])
    while todo:
        cur = todo.pop(0)
        for con in conns[cur]:
            if con not in visited:
                todo.append(con)
                visited.add(con)

    return visited

def main1(filename):
    f = open(filename, "r")
    conns = {}
    for line in f:
        src, trgts = line.strip().split(' <-> ')
        conns[int(src)] = list(map(int, trgts.split(', ')))

    visited = ff(conns, 0)
    print("p1: " + str(len(visited)))

    all_visited = set()
    group_count = 0
    for start in conns:
        if start not in all_visited:
            group_count += 1
            all_visited.update(ff(conns, start))

    print("p2: " + str(group_count))


if __name__ == "__main__":
    main1("input_t1")
    main1("input")
