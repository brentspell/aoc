import TinyQueue from 'tinyqueue'

function part1(data: string): number {
  return simulate(new State(data)).length
}

function part2(data: string): number {
  const state = new State(data)
  state.floors[0].add('elerium generator')
  state.floors[0].add('elerium microchip')
  state.floors[0].add('dilithium generator')
  state.floors[0].add('dilithium microchip')
  return simulate(state).length
}

function simulate(state: State): State[] {
  const queue = new TinyQueue(
    [state],
    (a: State, b: State) => a.cost() - b.cost()
  )
  const done: Set<string> = new Set()
  for (;;) {
    const state = queue.pop()
    if (!state) {
      break
    }

    // check for completion condition
    if (state.floors.slice(0, -1).every((f) => f.size == 0)) {
      return state.path
    }
    // enumerate next possible elevator moves
    const nextElevators = []
    if (state.elevator > 0) {
      // if we have moved everything above a level, don't go back down
      if (state.floors.slice(0, state.elevator).some((f) => f.size > 0)) {
        nextElevators.push(state.elevator - 1)
      }
    }
    if (state.elevator < state.floors.length - 1) {
      nextElevators.push(state.elevator + 1)
    }
    for (const nextElevator of nextElevators) {
      // enumerate next possible element moves
      const floor = [...state.floors[state.elevator]]
      for (let i = 0; i < floor.length; i++) {
        // move one element
        const next = new State(state)
        next.steps += 1
        next.path.push(next)
        next.elevator = nextElevator
        next.floors[state.elevator].delete(floor[i])
        next.floors[nextElevator].add(floor[i])
        // only expand valid, unvisited states
        if (next.isValid(state.elevator) && next.isValid(nextElevator)) {
          const d1 = next.stringify()
          if (!done.has(d1)) {
            done.add(d1)
            queue.push(next)
          }
        }
        // move two elements
        for (let j = i + 1; j < floor.length; j++) {
          const next = new State(state)
          next.steps += 1
          next.path.push(next)
          next.elevator = nextElevator
          next.floors[state.elevator].delete(floor[i])
          next.floors[nextElevator].add(floor[i])
          next.floors[state.elevator].delete(floor[j])
          next.floors[nextElevator].add(floor[j])
          // only expand valid, unvisited states
          if (next.isValid(state.elevator) && next.isValid(nextElevator)) {
            const d2 = next.stringify()
            if (!done.has(d2)) {
              done.add(d2)
              queue.push(next)
            }
          }
        }
      }
    }
  }
  return []
}

class State {
  path: State[]
  steps: number
  elevator: number
  floors: Set<string>[]

  constructor(data: string | State) {
    if (typeof data == 'string') {
      this.path = []
      this.steps = 0
      this.elevator = 0
      this.floors = data
        .replace(/-compatible/g, '')
        .split('\n')
        .map((l) => new Set(l.match(/\w+ (generator|microchip)/g) || []))
    } else {
      this.path = data.path.slice(0)
      this.steps = data.steps
      this.elevator = data.elevator
      this.floors = data.floors.map((s) => new Set(s))
    }
  }

  stringify(): string {
    let result = ''
    for (let i = this.floors.length - 1; i >= 0; i--) {
      result += i == this.elevator ? 'E' : '.'
      result += [...this.floors[i]].sort().join(' ')
      result += '\n'
    }
    result += '\n'
    return result
  }

  isValid(floorIndex: number): boolean {
    const floor = this.floors[floorIndex]
    const floorArray = [...floor]
    if (floorArray.some((e) => e.includes('generator'))) {
      for (const chip of floorArray.filter((e) => e.includes('microchip'))) {
        if (!floor.has(chip.replace('microchip', 'generator'))) {
          return false
        }
      }
    }
    return true
  }

  cost(): number {
    return this.steps * 50 + this.floors.reduce((a, f) => a * 10 + f.size, 0)
  }
}

export { part1, part2 }
