function part1(data: string): string {
  const buttons = ['123', '456', '789']
  let i = 1
  let j = 1
  let c = ''
  for (const l of data.split('\n')) {
    for (const d of l) {
      if (d == 'U') {
        i = Math.max(i - 1, 0)
      } else if (d == 'D') {
        i = Math.min(i + 1, buttons.length - 1)
      } else if (d == 'L') {
        j = Math.max(j - 1, 0)
      } else if (d == 'R') {
        j = Math.min(j + 1, buttons.length - 1)
      }
    }
    c += buttons[i][j]
  }
  return c
}

function part2(data: string): string {
  const buttons = ['  1  ', ' 234 ', '56789', ' ABC ', '  D  ']
  let i = 2
  let j = 0
  let c = ''
  for (const l of data.split('\n')) {
    for (const d of l) {
      let ni = i
      let nj = j
      if (d == 'U') {
        ni = Math.max(ni - 1, 0)
      } else if (d == 'D') {
        ni = Math.min(ni + 1, buttons.length - 1)
      } else if (d == 'L') {
        nj = Math.max(nj - 1, 0)
      } else if (d == 'R') {
        nj = Math.min(nj + 1, buttons.length - 1)
      }
      if (buttons[ni][nj] != ' ') {
        i = ni
        j = nj
      }
    }
    c += buttons[i][j]
  }
  return c
}

export { part1, part2 }
