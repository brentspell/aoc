function part1(data: string): number {
  return data
    .split('\n')
    .map((l) => l.split('-').map((x) => parseInt(x)))
    .sort((a, b) => a[0] - b[0])
    .reduce((min, [lo, hi]) => (min < lo ? min : Math.max(min, hi + 1)), 0)
}

function part2(data: string): number {
  const [sum, min] = data
    .split('\n')
    .map((l) => l.split('-').map((x) => parseInt(x)))
    .sort((a, b) => a[0] - b[0])
    .reduce(
      ([sum, min], [lo, hi]) => [
        sum + Math.max(lo - min, 0),
        Math.max(min, hi + 1),
      ],
      [0, 0]
    )
  return sum + 2 ** 32 - min
}

export { part1, part2 }
