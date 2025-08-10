input_string = input()

mismatched_pair = False
len = len(input_string)

for i in range(0, len // 2 + 1):
    if input_string[i] != input_string[len - i - 1]:
        if mismatched_pair:
            print("NO")
            exit(0)
        mismatched_pair = True

if mismatched_pair:
    print("YES")
elif len % 2 == 1:
    print("YES")
else:
    print("NO")
