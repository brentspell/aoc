import { part1, part2 } from '../day01'

describe('today', () => {
  it('runs part1', () => {
    expect(part1('R2, L3')).toBe(5)
    expect(part1('R2, R2, R2')).toBe(2)
    expect(part1('R5, L5, R5, R3')).toBe(12)
  })

  it('runs part2', () => {
    expect(part2('R8, R4, R4, R8')).toBe(4)
  })
})
