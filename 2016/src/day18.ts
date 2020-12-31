function part1(data: string): number {
  const params = data.split(',')
  return simulate(params[1], parseInt(params[0]))
}

function part2(data: string): number {
  return simulate(data.split(',')[1], 400000)
}

function simulate(data: string, length: number): number {
  const tiles = data.split('')
  let safe = (tiles.join('').match(/\./g) || []).length
  for (let i = 1; i < length; i++) {
    const prev = ['.'].concat(tiles).concat('.')
    for (let j = 0; j < tiles.length; j++) {
      const [l, c, r] = prev.slice(j)
      tiles[j] =
        (l == '^' && c == '^' && r == '.') ||
        (l == '.' && c == '^' && r == '^') ||
        (l == '^' && c == '.' && r == '.') ||
        (l == '.' && c == '.' && r == '^')
          ? '^'
          : '.'
    }
    safe += (tiles.join('').match(/\./g) || []).length
  }
  return safe
}

export { part1, part2 }
