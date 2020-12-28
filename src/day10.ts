function part1(data: string): number {
  return simulate(data.split('\n'))[0]
}

function part2(data: string): number {
  return simulate(data.split('\n'))[1]
}

function simulate(instructions: string[]): [number, number] {
  // first, assign all initial values to the bots
  // ignore all non-assignment instructions
  const bots: Record<string, number[]> = {}
  for (let i = 0; i < instructions.length; i++) {
    const [v, b] = (
      instructions[i].match(/value (\d+) goes to bot (\d+)/) || []
    )
      .slice(1)
      .map((x) => parseInt(x))
    if (v) {
      bots[b] = (bots[b] || []).concat([v]).sort((a, b) => a - b)
      instructions.splice(i--, 1)
    }
  }

  // next, repeatedly attempt to process assignment instructions where possible
  const outputs: Record<string, number> = {}
  let bot = 0
  while (instructions.length > 0) {
    for (let i = 0; i < instructions.length; i++) {
      const [s, lt, ln, ht, hn] = (
        instructions[i].match(
          /bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)/
        ) || []
      ).slice(1)

      // an assignment instruction can only be processed this round
      // if the source bot has two stored values and
      // the target bots have less than two stored values
      // or we are sending the values to outputs
      if (
        (bots[s] || []).length == 2 &&
        (lt == 'output' || (bots[ln] || []).length < 2) &&
        (ht == 'output' || (bots[hn] || []).length < 2)
      ) {
        if (lt == 'bot') {
          bots[ln] = (bots[ln] || []).concat([bots[s][0]]).sort((a, b) => a - b)
        } else {
          outputs[ln] = bots[s][0]
        }
        if (ht == 'bot') {
          bots[hn] = (bots[hn] || []).concat([bots[s][1]]).sort((a, b) => a - b)
        } else {
          outputs[hn] = bots[s][1]
        }
        bots[s] = []
        instructions.splice(i--, 1)
      }

      // check for the magic comparison values for the first part
      for (const [k, v] of Object.entries(bots)) {
        if (v[0] == 17 && v[1] == 61) {
          bot = parseInt(k)
        }
      }
    }
  }
  return [bot, outputs[0] * outputs[1] * outputs[2]]
}

export { part1, part2 }
