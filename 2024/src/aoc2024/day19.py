import collections


def part1(data: str) -> int:
    # count the  number of designs that have at least one match
    trie, designs = parse(data)
    return sum(trie.matches(d) > 0 for d in designs)


def part2(data: str) -> int:
    # count all possible matches
    trie, designs = parse(data)
    return sum(trie.matches(d) for d in designs)


def parse(data: str) -> tuple["Trie", list[str]]:
    s1, s2 = data.split("\n\n")
    patterns = s1.split(", ")
    designs = s2.splitlines()

    trie = Trie()
    for p in patterns:
        trie.insert(p)

    return trie, designs


class Trie:
    def __init__(self) -> None:
        self.children: dict[str, Trie] = {}
        self.text: str | None = None

    @property
    def is_key(self) -> bool:
        return self.text is not None

    def insert(self, text: str) -> None:
        node = self
        for char in text:
            if (child := node.children.get(char)) is None:
                child = node.children[char] = Trie()
            node = child
        node.text = text

    def matches(self, text: str) -> int:
        # maintain a multiset of each node,
        # which counts all possible paths to that node
        nodes = collections.Counter([self])
        for ch in text:
            children: collections.Counter[Trie] = collections.Counter()
            for node, paths in nodes.items():
                if (child := node.children.get(ch)) is not None:
                    # if we have reached a stored key node, start over at the root
                    if child.is_key:
                        children[self] += paths
                    # and continue from the current node
                    children[child] += paths

            nodes = children

        # accumulate all possible paths to leaves
        return sum(paths for node, paths in nodes.items() if node.is_key)
