from z3 import Int, Optimize, sat

def solve_buttons(target, buttons):
    n, m = len(target), len(buttons)

    B = [[0] * m for _ in range(n)]
    for j, btn in enumerate(buttons):
        for i in btn:
            B[i][j] += 1

    opt = Optimize()
    x = [Int(f"x{j}") for j in range(m)]

    # Non-negative constraints
    for var in x:
        opt.add(var >= 0)

    # Linear constraints
    for i in range(n):
        opt.add(sum(B[i][j] * x[j] for j in range(m)) == target[i])

    opt.minimize(sum(x))

    if opt.check() != sat:
        return None

    model = opt.model()
    return [model[v].as_long() for v in x]

def parse_list(s):
    s = s.strip("(){} ")
    return [] if not s else list(map(int, s.split(",")))

total = 0

with open("input") as f:
    for line in f:
        segments = line.split()
        buttons = [parse_list(b) for b in segments[1:-1]]
        target = parse_list(segments[-1])
        total += sum(solve_buttons(target, buttons))

print(total)