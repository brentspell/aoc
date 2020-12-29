import { part1 } from '../day18'

describe('day18', () => {
  it('runs part1', () => {
    expect(part1('3,..^^.')).toBe(6)
    expect(part1('10,.^^.^.^^^^')).toBe(38)
  })
})
