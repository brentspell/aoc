import * as crypto from 'crypto'

function part1(data: string): string {
  let pass = ''
  let j = 0
  for (let i = 0; i < 8; i++) {
    let hash = ''
    while (hash.slice(0, 5) != '00000')
      hash = crypto
        .createHash('md5')
        .update(data + (j++).toString())
        .digest('hex')

    pass += hash[5]
  }
  return pass
}

function part2(data: string): string {
  const pass = '        '.split('')
  let i = 0
  while (pass.includes(' ')) {
    let hash = ''
    while (hash.slice(0, 5) != '00000')
      hash = crypto
        .createHash('md5')
        .update(data + (i++).toString())
        .digest('hex')

    const index = parseInt(hash[5])
    if (pass[index] == ' ') pass[index] = hash[6]
  }
  return pass.join('')
}

export { part1, part2 }
