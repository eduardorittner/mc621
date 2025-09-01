import sys


def find_pisano_period(b):
    if b == 1:
        return 1
    a, b_val = 0, 1
    for i in range(b * b):
        a, b_val = b_val, (a + b_val) % b
        if a == 0 and b_val == 1:
            return i + 1
    return b * b


def fib_mod(n, b):
    if b == 1:
        return 0
    if n == 0:
        return 0
    if n == 1:
        return 1
    a, b_val = 0, 1
    for _ in range(2, int(n) + 1):
        a, b_val = b_val, (a + b_val) % b
    return b_val


def solve():
    case = 1
    for line in sys.stdin:
        parts = line.strip().split()
        if not parts:
            continue
        n = int(parts[0])
        b = int(parts[1])

        if n == 0 and b == 0:
            break

        pisano_period = find_pisano_period(b)
        effective_n = (n + 1) % pisano_period
        fib_n_plus_1_mod_b = fib_mod(effective_n, b)

        result = (2 * fib_n_plus_1_mod_b) % b
        final_result = (result - 1 + b) % b

        print(f"Case {case}: {n} {b}", final_result)
        case += 1


if __name__ == "__main__":
    solve()
