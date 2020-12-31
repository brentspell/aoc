function part1(data: string): number {
  return run(data, { a: 7, b: 0, c: 0, d: 0 })
}

function part2(data: string): number {
  return run(data, { a: 12, b: 0, c: 0, d: 0 })
}

function run(data: string, registers: Record<string, number>): number {
  const program = data.split('\n').map((l) => l.split(' '))
  let ip = 0
  while (ip < program.length) {
    // optimize [inc/dec, inc/dec, jnz <reg> -2] sequences
    // these increments/decrements can be converted to addition/subtraction
    const [i0, i1, i2] = program.slice(ip)
    if (
      i2 &&
      i2[0] == 'jnz' &&
      i2[2] == '-2' &&
      ['inc', 'dec'].includes(i0[0]) &&
      ['inc', 'dec'].includes(i1[0]) &&
      (i0[1] == i2[1] || i1[1] == i2[1])
    ) {
      const src = i2[1]
      const dst = i0[1] == src ? i1[1] : i0[1]
      const op = i0[1] == src ? i1[0] : i0[0]
      const mul = op == 'inc' ? 1 : -1
      registers[dst] += mul * Math.abs(registers[src])
      registers[src] = 0
      ip += 3
      continue
    }

    // otherwise, interpret the program normally
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
          const jmp = arg2 in registers ? registers[arg2] : parseInt(arg2)
          ip += jmp - 1
        }
        break
      }
      case 'tgl': {
        const mod = ip + registers[arg1] - 1
        if (mod >= 0 && mod < program.length) {
          const code = program[mod]
          code[0] =
            code.length == 2
              ? code[0] == 'inc'
                ? 'dec'
                : 'inc'
              : code[0] == 'jnz'
              ? 'cpy'
              : 'jnz'
        }
      }
    }
  }
  return registers['a']
}

export { part1, part2 }
