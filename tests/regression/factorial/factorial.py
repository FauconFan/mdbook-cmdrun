#!/usr/bin/python3

import sys

def factorial():
	a = 1
	n = 1
	
	yield a

	while True:
		n += 1
		a *= n
		yield a

def main():
	n = int(sys.argv[1])
	generator = factorial()
	li = [next(generator) for _ in range(n)]

	print("| n | n! |")
	print("|---|--:|")
	for n, fibn in enumerate(li):
		print(f"| {n + 1} | {fibn} |")

if __name__ == "__main__":
	main()
