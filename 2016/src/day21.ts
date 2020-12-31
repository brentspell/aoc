function part1(data: string): string {
  const split = data.split('\n')
  return scramble(split[0], split.slice(1))
}

function part2(data: string): string {
  const program = data.split('\n').slice(1)
  for (const p of permute('abcdefgh')) {
    if (scramble(p, program) == 'fbgdceah') {
      return p
    }
  }
  return ''
}

function scramble(start: string, program: string[]) {
  let pass = start.split('')
  for (const line of program) {
    const pswap = line.match(/swap position (\d+) with position (\d+)/)
    if (pswap) {
      const [i, j] = pswap.slice(1).map((s) => parseInt(s))
      ;[pass[i], pass[j]] = [pass[j], pass[i]]
    }

    const cswap = line.match(/swap letter (\w) with letter (\w)/)
    if (cswap) {
      const [i, j] = cswap.slice(1).map((c) => pass.indexOf(c))
      if (i >= 0 && j >= 0) {
        const t = pass[i]
        pass[i] = pass[j]
        pass[j] = t
      }
    }

    const prot = line.match(/rotate (left|right) (\d+) steps?/)
    if (prot) {
      if (parseInt(prot[2]) > pass.length) throw 'Error1'
      if (prot[1] == 'left') {
        pass = rotateLeft(pass, parseInt(prot[2]))
      } else {
        pass = rotateRight(pass, parseInt(prot[2]))
      }
    }

    const crot = line.match(/rotate based on position of letter (\w)/)
    if (crot) {
      const rots = pass.indexOf(crot[1])
      if (rots >= 0) {
        pass = rotateRight(pass, 1)
        pass = rotateRight(pass, rots)
        if (rots >= 4) {
          pass = rotateRight(pass, 1)
        }
      }
    }

    const rev = line.match(/reverse positions (\d+) through (\d+)/)
    if (rev) {
      const [i, j] = rev.slice(1).map((s) => parseInt(s))
      pass = pass
        .slice(0, i)
        .concat(pass.slice(i, j + 1).reverse())
        .concat(pass.slice(j + 1))
    }

    const move = line.match(/move position (\d+) to position (\d+)/)
    if (move) {
      const [i, j] = move.slice(1).map((s) => parseInt(s))
      const [c] = pass.splice(i, 1)
      pass.splice(j, 0, c)
    }
  }
  return pass.join('')
}

function rotateLeft(array: string[], count: number): string[] {
  return array.slice(count).concat(array.slice(0, count))
}

function rotateRight(array: string[], count: number): string[] {
  return array.slice(-count).concat(array.slice(0, -count))
}

function permute(str: string): string[] {
  const result = []
  if (str.length == 0) {
    result.push('')
  } else {
    const chars = str.split('')
    for (let i = 0; i < chars.length; i++) {
      const sub = [...chars]
      const [char] = sub.splice(i, 1)
      for (const p of permute(sub.join(''))) {
        result.push(char + p)
      }
    }
  }
  return result
}

export { part1, part2 }
