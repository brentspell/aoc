import TinyQueue from 'tinyqueue'

function part1(data: string): number {
  const [fav, xEnd, yEnd] = data.split(',').map((x) => parseInt(x))
  const queue = new TinyQueue(
    [[0, 1, 1]],
    (a: [number, number, number], b: [number, number, number]) => a[0] - b[0]
  )
  const done: Set<string> = new Set()
  for (let p = queue.pop(); p; p = queue.pop()) {
    const [c, x, y] = p
    if (x == xEnd && y == yEnd) {
      return c
    }
    done.add([x, y].toString())
    for (const [u, v] of neighbors(fav, x, y)) {
      if (!done.has([u, v].toString())) {
        queue.push([c + 1, u, v])
      }
    }
  }
  return 0
}

function part2(data: string): number {
  const [fav] = data.split(',').map((x) => parseInt(x))
  const queue = new TinyQueue(
    [[0, 1, 1]],
    (a: [number, number, number], b: [number, number, number]) => a[0] - b[0]
  )
  const done: Set<string> = new Set()
  for (let p = queue.pop(); p; p = queue.pop()) {
    const [c, x, y] = p
    done.add([x, y].toString())
    if (c < 50) {
      for (const [u, v] of neighbors(fav, x, y)) {
        if (!done.has([u, v].toString())) {
          queue.push([c + 1, u, v])
        }
      }
    }
  }
  return done.size
}

function neighbors(fav: number, x: number, y: number): [number, number][] {
  const neighbors: [number, number][] = []
  for (const [u, v] of [
    [x - 1, y],
    [x + 1, y],
    [x, y - 1],
    [x, y + 1],
  ]) {
    if (!isWall(fav, u, v)) {
      neighbors.push([u, v])
    }
  }
  return neighbors
}

function isWall(fav: number, x: number, y: number): boolean {
  return (
    x < 0 ||
    y < 0 ||
    popcount(x * x + 3 * x + 2 * x * y + y + y * y + fav) % 2 == 1
  )
}

function popcount(x: number): number {
  let b = 0
  while (x != 0) {
    b += x & 1
    x >>= 1
  }
  return b
}

export { part1, part2 }
