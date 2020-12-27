function part1(data: string): number {
  return data
    .split('\n')
    .map((l) => {
      for (const m of l.match(/\[.*?\]/g) || [])
        if (isAbba(m.slice(1, -1))) return 0
      for (const a of l.replace(/\[.*?\]/g, '|').split('|'))
        if (isAbba(a)) return Number(1)
      return 0
    })
    .reduce((a, x) => a + x)
}

function part2(data: string): number {
  return data
    .split('\n')
    .map((l) => {
      for (const a of l.replace(/\[.*?\]/g, '|').split('|'))
        for (let i = 0; i < a.length - 2; i++)
          if (a[i] == a[i + 2] && a[i] != a[i + 1])
            for (const m of l.match(/\[.*?\]/g) || [])
              if (m.includes([a[i + 1], a[i], a[i + 1]].join('')))
                return Number(1)
      return 0
    })
    .reduce((a, x) => a + x)
}

function isAbba(str: string): boolean {
  for (let i = 0; i < str.length - 3; i++) {
    const [a, b, c, d] = str.slice(i)
    if (a == d && b == c && a != b) return true
  }
  return false
}

export { part1, part2 }
