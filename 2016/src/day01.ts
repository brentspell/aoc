const orient: Record<string, string> = {
  NR: 'E',
  NL: 'W',
  SR: 'W',
  SL: 'E',
  ER: 'S',
  EL: 'N',
  WR: 'N',
  WL: 'S',
}

function part1(data: string): number {
  let x = 0
  let y = 0
  let z = 'N'
  for (const d of data.split(', ')) {
    z = orient[z + d[0]]
    const n = parseInt(d.substring(1))
    if (z == 'N') {
      y += n
    } else if (z == 'S') {
      y -= n
    } else if (z == 'E') {
      x += n
    } else if (z == 'W') {
      x -= n
    }
  }
  return Math.abs(x) + Math.abs(y)
}

function part2(data: string): number {
  let x = 0
  let y = 0
  let z = 'N'
  const visited = new Set()
  for (const d of data.split(', ')) {
    z = orient[z + d[0]]
    const n = parseInt(d.substring(1))
    for (let i = 0; i < n; i++) {
      visited.add([x, y].toString())
      if (z == 'N') {
        y += 1
      } else if (z == 'S') {
        y -= 1
      } else if (z == 'E') {
        x += 1
      } else if (z == 'W') {
        x -= 1
      }
      if (visited.has([x, y].toString())) {
        return Math.abs(x) + Math.abs(y)
      }
    }
  }
  return Math.abs(x) + Math.abs(y)
}

export { part1, part2 }
