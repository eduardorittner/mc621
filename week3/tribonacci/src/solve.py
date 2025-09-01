import sys

MOD = 1_000_000_009


def matrix_multiply(A, B):
    C = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
    for i in range(3):
        for j in range(3):
            for k in range(3):
                C[i][j] = (C[i][j] + A[i][k] * B[k][j]) % MOD
    return C


def matrix_power(A, exp):
    res = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    while exp > 0:
        if exp % 2 == 1:
            res = matrix_multiply(res, A)
        A = matrix_multiply(A, A)
        exp //= 2
    return res


def solve():
    T = [[1, 1, 1], [1, 0, 0], [0, 1, 0]]

    initial_v = [[2], [1], [0]]

    for line in sys.stdin:
        n = int(line.strip())
        if n == 0:
            break

        if n == 1:
            print(0)
            continue
        if n == 2:
            print(1)
            continue
        if n == 3:
            print(2)
            continue

        T_pow = matrix_power(T, n - 3)

        result = (
            T_pow[0][0] * initial_v[0][0]
            + T_pow[0][1] * initial_v[1][0]
            + T_pow[0][2] * initial_v[2][0]
        ) % MOD

        print(result)


if __name__ == "__main__":
    solve()
