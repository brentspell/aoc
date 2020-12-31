import { part1, part2 } from '../day09'

describe('day09', () => {
  it('runs part1', () => {
    expect(part1('ADVENT')).toBe(6)
    expect(part1('A(1x5)BC')).toBe(7)
    expect(part1('(3x3)XYZ')).toBe(9)
    expect(part1('A(2x2)BCD(2x2)EFG')).toBe(11)
    expect(part1('(6x1)(1x3)A')).toBe(6)
    expect(part1('X(8x2)(3x3)ABCY')).toBe(18)
  })
  it('runs part2', () => {
    expect(part2('(3x3)XYZ')).toBe(9)
    expect(part2('X(8x2)(3x3)ABCY')).toBe(20)
    expect(part2('(27x12)(20x12)(13x14)(7x10)(1x12)A')).toBe(241920)
    expect(
      part2('(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN')
    ).toBe(445)
  })
})
