#!/usr/bin/python3

import sys

def fibonacci():
	a = 0
	b = 1
	
	yield a
	yield b

	while True:
		a, b = b, a + b
		yield b

def main():
	for n in sys.argv[1:]:
		n = int(n)
		generator = fibonacci()
		li = [next(generator) for _ in range(n)]

		print(f"# fibonacci up to {n}")
		print("| n | fib(n) |")
		print("|---|--:|")
		for n, fibn in enumerate(li):
			print(f"| {n + 1} | {fibn} |")

if __name__ == "__main__":
	main()
