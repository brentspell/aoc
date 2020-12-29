function part1(data: string): string {
  const parts = data.split(',')
  let input = parts[0]
  const length = parseInt(parts[1] || '272')
  while (input.length < length) {
    input +=
      '0' +
      input
        .split('')
        .reverse()
        .join('')
        .replace(/0/g, 'z')
        .replace(/1/g, '0')
        .replace(/z/g, '1')
  }
  input = input.slice(0, length)

  let sum = ''
  while (sum.length % 2 == 0) {
    sum = ''
    for (let i = 0; i < input.length; i += 2) {
      sum += input[i] == input[i + 1] ? '1' : '0'
    }
    input = sum
  }
  return sum
}

function part2(data: string): string {
  let input = data
  const length = 35651584
  while (input.length < length) {
    input +=
      '0' +
      input
        .split('')
        .reverse()
        .join('')
        .replace(/0/g, 'z')
        .replace(/1/g, '0')
        .replace(/z/g, '1')
  }
  input = input.slice(0, length)

  let sum = ''
  while (sum.length % 2 == 0) {
    sum = ''
    for (let i = 0; i < input.length; i += 2) {
      sum += input[i] == input[i + 1] ? '1' : '0'
    }
    input = sum
  }
  return sum
}

export { part1, part2 }
