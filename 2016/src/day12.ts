function part1(data: string): number {
  return run(data, { a: 0, b: 0, c: 0, d: 0 })
}

function part2(data: string): number {
  return run(data, { a: 0, b: 0, c: 1, d: 0 })
}

function run(data: string, registers: Record<string, number>): number {
  const program = data.split('\n').map((l) => l.split(' '))
  let ip = 0
  while (ip < program.length) {
    const [op, arg1, arg2] = program[ip++]
    switch (op) {
      case 'cpy': {
        registers[arg2] = arg1 in registers ? registers[arg1] : parseInt(arg1)
        break
      }
      case 'inc': {
        registers[arg1]++
        break
      }
      case 'dec': {
        registers[arg1]--
        break
      }
      case 'jnz': {
        if (registers[arg1] != 0) {
          ip += parseInt(arg2) - 1
        }
        break
      }
    }
  }
  return registers['a']
}

export { part1, part2 }
