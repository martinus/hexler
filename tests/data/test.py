import sys

for code in range(30, 38):
    sys.stdout.write(f"\u001b[{code}m{code} ")
print()
for code in range(90, 98):
    sys.stdout.write(f"\u001b[{code}m{code} ")
sys.stdout.write("\u001b[0m0 ")
print()

for i in range(0, 16):
    for j in range(0, 16):
        code = str(i * 16 + j)
        sys.stdout.write(f"\u001b[38;5;{code}m{code.rjust(5)}")
    print("\u001b[0m")
