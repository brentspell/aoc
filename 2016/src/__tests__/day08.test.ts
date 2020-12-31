import { part1 } from '../day08'

const data = [
  'rect 3x2',
  'rotate column x=1 by 1',
  'rotate row y=0 by 4',
  'rotate column x=1 by 1',
].join('\n')

describe('day08', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(6)
  })
})
