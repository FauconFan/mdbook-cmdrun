const prices = {
  apples: 1.99,
  bananas: 1.89
}

console.log(prices[process.argv[2]] ?? prices[apples])
