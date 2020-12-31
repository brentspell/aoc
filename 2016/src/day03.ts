function part1(data: string): number {
  return data
    .split('\n')
    .map((l) => {
      const [a, b, c] = l
        .split(' ')
        .map((s) => parseInt(s))
        .sort((a, b) => a - b)
      return Number(a + b > c)
    })
    .reduce((a, x) => a + x)
}

function part2(data: string): number {
  const num = data
    .split('\n')
    .map((l) => l.split(' ').map((s) => parseInt(s)))
    .flat()
  let valid = 0
  for (let i = 0; i < num.length; i += 9) {
    for (let j = 0; j < 3; j++) {
      const [a, b, c] = [num[i + j], num[i + 3 + j], num[i + 6 + j]].sort(
        (a, b) => a - b
      )
      if (a + b > c) valid++
    }
  }
  return valid
}

export { part1, part2 }
