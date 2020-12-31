function part1(data: string): number {
  return decompress(data, false)
}

function part2(data: string): number {
  return decompress(data, true)
}

function decompress(str: string, expand: boolean): number {
  let result = 0
  for (let j = 0; j < str.length; j++) {
    if (str[j] != '(') {
      result++
    } else {
      let expr = ''
      while (str[++j] != ')') {
        expr += str[j]
      }
      j++
      const [length, repeat] = expr.split('x').map((x) => parseInt(x))
      if (expand) {
        result += decompress(str.slice(j, j + length), expand) * repeat
      } else {
        result += length * repeat
      }
      j += length - 1
    }
  }
  return result
}

export { part1, part2 }
