import { part1 } from '../day25'

const data = ['cpy a d', 'cpy 7 c', 'cpy 362 b'].join('\n')

describe('day25', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe(196)
  })
})
