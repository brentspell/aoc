import { part1, part2 } from '../day23'

const data = [
  'cpy 2 a',
  'tgl a',
  'tgl a',
  'tgl a',
  'cpy 1 a',
  'dec a',
  'dec a',
].join('\n')

describe('day23', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(3)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(3)
  })
})
