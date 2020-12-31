import { part1, part2 } from '../day12'

const data = ['cpy 41 a', 'inc a', 'inc a', 'dec a', 'jnz a 2', 'dec a'].join(
  '\n'
)

describe('day12', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(42)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(42)
  })
})
