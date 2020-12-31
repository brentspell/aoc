function part1(data: string): number {
  return process(data)
    .map((r) => r.map((c) => (c == '#' ? Number(1) : 0)))
    .flat()
    .reduce((a, x) => a + x)
}

function part2(data: string): string {
  const screen = process(data)
  for (const r of screen) console.log(r.join(''))
  return ''
}

function process(data: string): string[][] {
  const screen = [
    '.'.repeat(50).split(''),
    '.'.repeat(50).split(''),
    '.'.repeat(50).split(''),
    '.'.repeat(50).split(''),
    '.'.repeat(50).split(''),
    '.'.repeat(50).split(''),
  ]
  for (const l of data.split('\n')) {
    const rect = l.match(/rect (\d+)x(\d+)/)
    if (rect) {
      const [c, r] = rect.slice(1).map((x) => parseInt(x))
      for (let i = 0; i < r; i++) for (let j = 0; j < c; j++) screen[i][j] = '#'
    }
    const rotrow = l.match(/rotate row y=(\d+) by (\d+)/)
    if (rotrow) {
      const [r, n] = rotrow.slice(1).map((x) => parseInt(x))
      screen[r] = rotate(screen[r], n)
    }
    const rotcol = l.match(/rotate column x=(\d+) by (\d+)/)
    if (rotcol) {
      const [c, n] = rotcol.slice(1).map((x) => parseInt(x))
      const col = rotate(
        screen.map((r) => r[c]),
        n
      )
      for (const r of screen) r[c] = col.shift() || ''
    }
  }
  return screen
}

function rotate(array: string[], count: number): string[] {
  return array.slice(-count).concat(array.slice(0, -count))
}

export { part1, part2 }
