import TinyQueue from 'tinyqueue'

function part1(data: string): number {
  return search(map(data), false)
}

function part2(data: string): number {
  return search(map(data), true)
}

// first, we must convert the maze into a weighted graph
// this is done by running an instance of BFS
// for each point of interest, in order to compute all
// pairwise costs
function map(data: string) {
  const maze = data.split('\n')

  // create a map of per-node costs for every maze point and
  // create a BFS queue for each point of interest
  const done = maze.map((r) => r.split('').map(() => new Map()))
  const queues: [number, number, number][][] = []
  for (let i = 0; i < maze.length; i++) {
    for (let j = 0; j < maze[i].length; j++) {
      if (maze[i][j].match(/\d/)) {
        queues.push([[i, j, 0]])
      }
    }
  }

  // construct the complete graph of costs between point pairs
  const graph = queues.map(() => queues.map(() => Number.MAX_SAFE_INTEGER))
  while (!graph.every((r) => r.every((c) => c < Number.MAX_SAFE_INTEGER))) {
    // run a single step of BFS for each point every iteration
    for (let i = 0; i < queues.length; i++) {
      const queue = queues[i]
      const state = queue.shift()
      if (state) {
        const [x, y, step] = state
        // if we reach a point in the maze that has been reached
        // by another BFS, we can combine our steps as a candidate cost
        done[x][y].set(i, step)
        for (const [j, pets] of done[x][y].entries()) {
          graph[i][j] = graph[j][i] = Math.min(graph[i][j], step + pets)
        }
        // continue the BFS for all neighboring maze points
        const neighbors = [
          [x - 1, y],
          [x + 1, y],
          [x, y - 1],
          [x, y + 1],
        ]
        for (const [x, y] of neighbors) {
          if (!done[x][y].has(i) && maze[x][y] != '#') {
            queue.push([x, y, step + 1])
          }
        }
        // if all edge costs have been calculated, we can stop this BFS
        if (graph[i].every((c) => c < Number.MAX_SAFE_INTEGER)) {
          queue.length = 0
        }
      }
    }
  }

  return graph
}

function search(graph: number[][], loop: boolean): number {
  // duplicate node 0 vertices as node N if looping back to start
  if (loop) {
    for (const row of graph) {
      row.push(row[0])
    }
    graph.push(graph[0])
  }

  // use dijkstra to find the minimum cost path through the graph
  type State = [number[], number]
  const queue = new TinyQueue([[[0], 0]], (a: State, b: State) => a[1] - b[1])
  for (;;) {
    const state = queue.pop()
    if (!state) {
      break
    }
    const [path, cost] = state
    const [i] = path
    // if we have traversed all points, we have the shortest path
    // if we are looping, ignore paths that don't end with the last node
    if (path.length == graph.length) {
      if (!loop || i == graph.length - 1) {
        return cost
      }
    }
    // continue the search at all unreached nodes,
    // since the graph is fully connected
    for (let j = 0; j < graph[i].length; j++) {
      if (!path.includes(j)) {
        queue.push([[j].concat(path), cost + graph[i][j]])
      }
    }
  }

  return -1
}

export { part1, part2 }
