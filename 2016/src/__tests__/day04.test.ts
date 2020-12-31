import { part1, part2 } from '../day04'

describe('day04', () => {
  it('runs part1', () => {
    const data = [
      'aaaaa-bbb-z-y-x-123[abxyz]',
      'a-b-c-d-e-f-g-h-987[abcde]',
      'not-a-real-room-404[oarel]',
      'totally-real-room-200[decoy]',
    ].join('\n')
    expect(part1(data)).toBe(1514)
  })

  it('runs part2', () => {
    const data = [
      'aaaaa-bbb-z-y-x-123[abxyz]',
      'a-b-c-d-e-f-g-h-987[abcde]',
      'not-a-real-room-404[oarel]',
      'totally-real-room-200[decoy]',
      'ijmockjgz-jwezxo-nojmvbz-993[jozmb]',
    ].join('\n')
    expect(part2(data)).toBe(993)
  })
})
