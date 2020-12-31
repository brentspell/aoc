import { part1, part2 } from '../day24'

const data = [
  '###########',
  '#0.1.....2#',
  '#.#######.#',
  '#4.......3#',
  '###########',
].join('\n')

describe('day24', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(14)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(20)
  })
})
