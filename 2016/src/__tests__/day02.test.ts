import { part1, part2 } from '../day02'

const data = ['ULL', 'RRDDD', 'LURDL', 'UUUUD'].join('\n')

describe('day02', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe('1985')
  })

  it('runs part2', () => {
    expect(part2(data)).toBe('5DB3')
  })
})
