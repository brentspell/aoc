import { part1, part2 } from '../day07'

describe('day07', () => {
  it('runs part1', () => {
    const data = [
      'abba[mnop]qrst',
      'abcd[bddb]xyyx',
      'aaaa[qwer]tyui',
      'ioxxoj[asdfgh]zxcvbn',
    ].join('\n')
    expect(part1(data)).toBe(2)
  })

  it('runs part2', () => {
    const data = [
      'aba[bab]xyz',
      'xyx[xyx]xyx',
      'aaa[kek]eke',
      'zazbz[bzb]cdb',
    ].join('\n')
    expect(part2(data)).toBe(3)
  })
})
