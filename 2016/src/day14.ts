import * as crypto from 'crypto'

function part1(data: string): number {
  const cands: Map<number, string> = new Map()
  const results: number[] = []
  for (let i = 0; results.length < 64 || cands.size > 0; i++) {
    const hash = crypto
      .createHash('md5')
      .update(data + i.toString())
      .digest('hex')
      .toLowerCase()
    for (const k of [...cands.keys()].sort((a, b) => a - b)) {
      if (i - k >= 1000) {
        cands.delete(k)
      } else if (hash.includes((cands.get(k) || '').repeat(5))) {
        cands.delete(k)
        results.push(k)
      }
    }
    if (results.length < 64) {
      for (let j = 0; j < hash.length - 2; j++) {
        if (hash[j] == hash[j + 1] && hash[j + 1] == hash[j + 2]) {
          cands.set(i, hash[j])
          break
        }
      }
    }
  }
  return results.sort((a, b) => a - b)[63]
}

function part2(data: string): number {
  const cands: Map<number, string> = new Map()
  const results: number[] = []
  for (let i = 0; results.length < 64 || cands.size > 0; i++) {
    let hash = crypto
      .createHash('md5')
      .update(data + i.toString())
      .digest('hex')
      .toLowerCase()
    for (let h = 0; h < 2016; h++)
      hash = crypto.createHash('md5').update(hash).digest('hex').toLowerCase()
    for (const k of [...cands.keys()].sort((a, b) => a - b)) {
      if (i - k >= 1000) {
        cands.delete(k)
      } else if (hash.includes((cands.get(k) || '').repeat(5))) {
        cands.delete(k)
        results.push(k)
      }
    }
    if (results.length < 64) {
      for (let j = 0; j < hash.length - 2; j++) {
        if (hash[j] == hash[j + 1] && hash[j + 1] == hash[j + 2]) {
          cands.set(i, hash[j])
          break
        }
      }
    }
  }
  return results.sort((a, b) => a - b)[63]
}

export { part1, part2 }
