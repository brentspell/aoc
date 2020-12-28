import fs from 'fs'
import { Solution } from './types'
import * as day01 from './day01'
import * as day02 from './day02'
import * as day03 from './day03'
import * as day04 from './day04'
import * as day05 from './day05'
import * as day06 from './day06'
import * as day07 from './day07'
import * as day08 from './day08'
import * as day09 from './day09'
import * as day10 from './day10'

const days: Record<string, Solution> = {
  '01': day01,
  '02': day02,
  '03': day03,
  '04': day04,
  '05': day05,
  '06': day06,
  '07': day07,
  '08': day08,
  '09': day09,
  '10': day10,
}

function run(name: string): void {
  const day = days[name]
  const data = fs.readFileSync(`data/day${name}.txt`, 'utf-8').trim()
  console.log(`day ${name}`)
  console.log(`  part 1: ${day.part1(data)}`)
  console.log(`  part 2: ${day.part2(data)}`)
}

if (process.argv.length == 3) {
  run(process.argv[2])
} else {
  for (const day in days) {
    run(day)
  }
}
