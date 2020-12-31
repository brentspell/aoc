function part1(data: string): number {
  return solve(parse(data))
}

function part2(data: string): number {
  const discs = parse(data)
  discs.push([11, 0])
  return solve(discs)
}

function parse(data: string): number[][] {
  const regex = /Disc #. has (\d+) positions; at time=0, it is at position (\d+)./
  return data
    .split('\n')
    .map((l) => (l.match(regex) || []).slice(1).map((s) => parseInt(s)))
}

// sieve method for solving the chinese remainder theorem
// find x, such that (x + pi + i) % ni == 0 for all ni, pi
function solve(discs: number[][]): number {
  let n = 1
  let x = 0
  let i = 0
  for (const [ni, pi] of discs) {
    i++
    while ((x + pi + i) % ni != 0) {
      x += n
    }
    n *= ni
  }
  return x
}

export { part1, part2 }
