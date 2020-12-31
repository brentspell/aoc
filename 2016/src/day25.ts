// after disassembling the program, it is clear that the
// answer + a constant (b*c assigned on lines 2/3) must
// have alternating 0/1 bits in its binary representation
function part1(data: string): number {
  // retrieve the b/c constants and multiply them
  const base = data
    .split('\n')
    .slice(1, 3)
    .map((s) => parseInt(s.split(' ')[1]))
    .reduce((a, x) => a * x)
  // find n such that base + n gives you alternating 0/1 bits
  let result = 0
  for (let i = 0; i < Math.log2(base) + 1; i++) {
    if (((base + result) & (1 << i)) >> i != i % 2) {
      result |= 1 << i
    }
  }
  return result
}

function part2(): number {
  return 0
}

export { part1, part2 }
