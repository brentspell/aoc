import fs from 'fs'
import { Solution } from './types'
import * as day01 from './day01'

const days: Record<string, Solution> = { '01': day01 }

function run(name: string): void {
  const day = days[name]
  const data = fs.readFileSync(`data/day${name}.txt`, 'utf-8')
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
