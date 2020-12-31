function part1(data: string): number {
  let sum = 0
  for (const l of data.split('\n')) {
    const s = l.split('-')
    const name = s.slice(0, -1).join('')
    const [id, check] = s[s.length - 1].slice(0, -1).split('[')
    const freq: Record<string, number> = {}
    for (const c of name) freq[c] = (freq[c] || 0) + 1
    const checked = Object.entries(freq)
      .sort(([k1, v1], [k2, v2]) => v2 - v1 || k1.localeCompare(k2))
      .map(([k]) => k)
      .slice(0, 5)
      .join('')
    if (checked == check) sum += parseInt(id)
  }
  return sum
}

function part2(data: string): number {
  for (const l of data.split('\n')) {
    const s = l.split('-')
    const id = parseInt(s[s.length - 1].split('[')[0])
    const a = 'a'.charCodeAt(0)
    const name = s
      .slice(0, -1)
      .map((s) =>
        s
          .split('')
          .map((c) =>
            String.fromCharCode(((c.charCodeAt(0) - a + id) % 26) + a)
          )
          .join('')
      )
      .join(' ')
    if (name == 'northpole object storage') return id
  }
  return 0
}

export { part1, part2 }
