import { part1, part2 } from '../day17'

describe('day17', () => {
  it('runs part1', () => {
    expect(part1('ihgpwlah')).toBe('DDRRRD')
    expect(part1('kglvqrro')).toBe('DDUDRLRRUDRD')
    expect(part1('ulqzkmiv')).toBe('DRURDRUDDLLDLUURRDULRLDUUDDDRR')
  })
  it('runs part2', () => {
    expect(part2('ihgpwlah')).toBe(370)
    expect(part2('kglvqrro')).toBe(492)
    expect(part2('ulqzkmiv')).toBe(830)
  })
})
