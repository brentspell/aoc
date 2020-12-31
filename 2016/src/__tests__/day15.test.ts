import { part1, part2 } from '../day15'

const data = [
  'Disc #1 has 5 positions; at time=0, it is at position 4.',
  'Disc #2 has 2 positions; at time=0, it is at position 1.',
].join('\n')

describe('day15', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(5)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(85)
  })
})
