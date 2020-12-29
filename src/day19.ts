function part1(data: string): number {
  // add all the elves to a circular linked list
  let node = new Node(1)
  for (let i = 1; i < parseInt(data); i++) {
    node.prev.append(new Node(i + 1))
  }
  // remove the left neighbor each turn until only one elf remains
  while (node.next != node) {
    node.next.remove()
    node = node.next
  }
  return node.id
}

function part2(data: string): number {
  // add all the elves to the circular linked list
  // advance a slow node every other insert to find the other side
  const count = parseInt(data)
  let node = new Node(1)
  let slow = node
  for (let i = 1; i < count; i++) {
    node.prev.append(new Node(i + 1))
    if (i % 2 == 1) {
      slow = slow.next
    }
  }
  // remove the nodes on the other side, advancing by one
  // or two nodes, depending on the parity of the count
  node = slow
  let skip = count % 2 == 1
  while (node.next != node) {
    node = node.next
    node.prev.remove()
    if (skip) {
      node = node.next
    }
    skip = !skip
  }
  return node.id
}

class Node {
  id: number
  next: Node
  prev: Node

  constructor(id: number) {
    this.id = id
    this.next = this
    this.prev = this
  }

  append(other: Node) {
    other.prev = this
    other.next = this.next
    this.next.prev = other
    this.next = other
    return this
  }

  remove() {
    this.next.prev = this.prev
    this.prev.next = this.next
    this.next = this
    this.prev = this
    return this
  }
}

export { part1, part2 }
