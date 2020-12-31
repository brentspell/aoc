import { part1, part2 } from '../day19'

describe('day19', () => {
  it('runs part1', () => {
    expect(part1('5')).toBe(3)
  })
  it('runs part2', () => {
    expect(part2('5')).toBe(2)
    expect(part2('6')).toBe(3)
  })
})
