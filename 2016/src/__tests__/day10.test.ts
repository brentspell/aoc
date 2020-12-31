import { part1, part2 } from '../day10'

const data = [
  'value 17 goes to bot 2',
  'bot 2 gives low to bot 1 and high to bot 0',
  'value 61 goes to bot 1',
  'bot 1 gives low to output 1 and high to bot 0',
  'bot 0 gives low to output 2 and high to output 0',
  'value 18 goes to bot 2',
].join('\n')

describe('day10', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(1)
  })
  it('runs part2', () => {
    expect(part2(data)).toBe(18666)
  })
})
