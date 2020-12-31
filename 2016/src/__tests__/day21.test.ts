import { part1, part2 } from '../day21'

const data = [
  'abcde',
  'swap position 4 with position 0',
  'swap letter d with letter b',
  'reverse positions 0 through 4',
  'rotate left 1 step',
  'move position 1 to position 4',
  'move position 3 to position 0',
  'rotate based on position of letter b',
  'rotate based on position of letter d',
].join('\n')

describe('day21', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe('decab')
  })
  it('runs part2', () => {
    expect(part2(data)).toBe('efghdabc')
  })
})
