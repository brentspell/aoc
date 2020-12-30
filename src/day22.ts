import TinyQueue from 'tinyqueue'

interface State {
  nodes: NodeMap
  goal: string
  hole: string
  step: number
}

type NodeMap = Record<string, Node>

interface Node {
  id: string
  x: number
  y: number
  used: number
  avail: number
}

function part1(data: string): number {
  const nodes: Node[] = data.split('\n').slice(2).map(parse)
  let result = 0
  for (let i = 0; i < nodes.length; i++) {
    for (let j = 0; j < nodes.length; j++) {
      if (i != j && nodes[i].used > 0 && nodes[i].used <= nodes[j].avail) {
        result++
      }
    }
  }
  return result
}

function part2(data: string): number {
  const nodes: NodeMap = data
    .split('\n')
    .slice(2)
    .map(parse)
    .reduce((o, n) => ({ ...o, [n.id]: n }), {})
  const goal = Object.values(nodes).reduce((a, n) => (n.x > a.x ? n : a)).id
  const hole = Object.values(nodes).filter((n) => n.used == 0)[0].id

  // A* heuristic search (steps only, ignore paths)
  const queue = new TinyQueue(
    [{ nodes, goal, hole, step: 0 }],
    (a: State, b: State) => cost(a) - cost(b)
  )
  const done: Set<string> = new Set()
  for (;;) {
    const state = queue.pop()
    if (!state) {
      break
    }
    if (state.goal == '0,0') {
      return state.step
    }
    done.add(`${state.goal}-${state.hole}`)
    for (const neighbor of neighbors(state)) {
      if (!done.has(`${neighbor.goal}-${neighbor.hole}`)) {
        queue.push(neighbor)
      }
    }
  }
  return 0
}

function neighbors(state: State): State[] {
  const { nodes, goal, hole, step } = state
  const result: State[] = []
  for (let i = -1; i <= 1; i++) {
    for (let j = -1; j <= 1; j++) {
      if (i * j == 0 && i != j) {
        const node = `${nodes[hole].x + i},${nodes[hole].y + j}`
        if ((nodes[node] || {}).used < nodes[hole].avail) {
          const next = clone(nodes)
          next[hole].used += next[node].used
          next[hole].avail -= next[node].used
          next[node].avail += next[node].used
          next[node].used = 0
          result.push({
            nodes: next,
            goal: node == goal ? hole : goal,
            hole: node,
            step: step + 1,
          })
        }
      }
    }
  }
  return result
}

function cost(state: State): number {
  // A* heuristic
  // we are primarily trying to get the goal to the origin,
  // but we need the subgoal of getting the empty disk to the goal
  const target = state.nodes['0,0']
  const goal = state.nodes[state.goal]
  const hole = state.nodes[state.hole]
  return 10 * distance(goal, target) + distance(hole, goal) + state.step
}

function distance(a: Node, b: Node): number {
  return Math.abs(a.x - b.x) + Math.abs(a.y - b.y)
}

function parse(line: string): Node {
  const m = line.match(/\/dev\/grid\/node-x(\d+)-y(\d+) +\d+T +(\d+)T +(\d+)T/)
  const [x, y, used, avail] = (m || []).slice(1)
  return {
    id: `${x},${y}`,
    x: parseInt(x),
    y: parseInt(y),
    used: parseInt(used),
    avail: parseInt(avail),
  }
}

function clone<T>(object: T): T {
  return JSON.parse(JSON.stringify(object))
}

export { part1, part2 }
