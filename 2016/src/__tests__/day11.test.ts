import { part1, part2 } from '../day11'

const data = [
  'The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.',
  'The second floor contains a hydrogen generator.',
  'The third floor contains a lithium generator.',
  'The fourth floor contains nothing relevant.',
].join('\n')

describe('day11', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(11)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(0)
  })
})
