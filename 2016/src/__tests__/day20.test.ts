import { part1, part2 } from '../day20'

const data = ['5-8', '0-2', '4-7'].join('\n')

describe('day20', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(3)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(4294967288)
  })
})
