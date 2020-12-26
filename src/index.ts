const days: Record<string, Record<string, () => number>> = {}

function run(name: string): void {
  const day = days[name]
  console.log(`day ${name}`)
  console.log(`  part 1: ${day.part1()}`)
  console.log(`  part 2: ${day.part2()}`)
}

if (process.argv.length == 3) {
   run(process.argv[2])
} else {
  for (const day in days) {
    run(day)
  }
}
