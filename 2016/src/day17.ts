import * as crypto from 'crypto'

function part1(data: string): string {
  const queue: [string, number, number][] = []
  const done: Set<string> = new Set()
  for (;;) {
    const [p, x, y]: [string, number, number] = queue.shift() || ['', 0, 0]
    if (x == 3 && y == 3) {
      return p
    }
    done.add(p)
    const hash = crypto
      .createHash('md5')
      .update(data + p)
      .digest('hex')
      .toLowerCase()
    const neighbors: [string, number, number][] = [
      ['L', x - 1, y],
      ['R', x + 1, y],
      ['U', x, y - 1],
      ['D', x, y + 1],
    ]
    for (const [d, u, v] of neighbors) {
      const np = p + d
      if (!done.has(np) && !isBlocked(hash, d, u, v)) {
        queue.push([np, u, v])
      }
    }
  }
  return ''
}

function part2(data: string): number {
  const queue: [string, number, number][] = [['', 0, 0]]
  const done: Set<string> = new Set()
  let longest = 0
  while (queue.length > 0) {
    const [p, x, y]: [string, number, number] = queue.shift() || ['', 0, 0]
    if (x == 3 && y == 3) {
      longest = Math.max(longest, p.length)
    } else {
      done.add(p)
      const hash = crypto
        .createHash('md5')
        .update(data + p)
        .digest('hex')
        .toLowerCase()
      const neighbors: [string, number, number][] = [
        ['L', x - 1, y],
        ['R', x + 1, y],
        ['U', x, y - 1],
        ['D', x, y + 1],
      ]
      for (const [d, u, v] of neighbors) {
        const np = p + d
        if (!done.has(np) && !isBlocked(hash, d, u, v)) {
          queue.push([np, u, v])
        }
      }
    }
  }
  return longest
}

function isBlocked(hash: string, dir: string, x: number, y: number): boolean {
  return (
    x < 0 ||
    x > 3 ||
    y < 0 ||
    y > 3 ||
    !'bcdef'.includes(hash['UDLR'.indexOf(dir)])
  )
}

export { part1, part2 }
