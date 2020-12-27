import { part1, part2 } from '../day06'

const data = [
  'eedadn',
  'drvtee',
  'eandsr',
  'raavrd',
  'atevrs',
  'tsrnev',
  'sdttsa',
  'rasrtv',
  'nssdts',
  'ntnada',
  'svetve',
  'tesnvt',
  'vntsnd',
  'vrdear',
  'dvrsen',
  'enarar',
].join('\n')

describe('day06', () => {
  it('runs part1', () => {
    expect(part1(data)).toBe('easter')
  })

  it('runs part2', () => {
    expect(part2(data)).toBe('advent')
  })
})
