function part1(data: string): string {
  const lines = data.split('\n')
  let message = ''
  for (let i = 0; i < lines[0].length; i++) {
    const freqs: Record<string, number> = {}
    for (const l of lines) {
      freqs[l[i]] = (freqs[l[i]] || 0) + 1
    }
    message += Object.entries(freqs).reduce(([k1, v1], [k2, v2]) =>
      v2 > v1 ? [k2, v2] : [k1, v1]
    )[0]
  }
  return message
}

function part2(data: string): string {
  const lines = data.split('\n')
  let message = ''
  for (let i = 0; i < lines[0].length; i++) {
    const freqs: Record<string, number> = {}
    for (const l of lines) {
      freqs[l[i]] = (freqs[l[i]] || 0) + 1
    }
    message += Object.entries(freqs).reduce(([k1, v1], [k2, v2]) =>
      v2 < v1 ? [k2, v2] : [k1, v1]
    )[0]
  }
  return message
}

export { part1, part2 }
