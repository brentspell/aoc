import { part1, part2 } from '../day03'

describe('day03', () => {
  it('runs part1', () => {
    const data = '3 4 5\n5 10 25'
    expect(part1(data)).toBe(1)
  })

  it('runs part2', () => {
    const data = [
      '101 301 501',
      '102 302 502',
      '103 303 503',
      '201 401 601',
      '202 402 602',
      '203 403 603',
    ].join('\n')
    expect(part2(data)).toBe(6)
  })
})
