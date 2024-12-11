import collections


def part1(data: str) -> int:
    stones = data.split()
    for _ in range(25):
        new = []
        for stone in stones:
            if stone == "0":
                new.append("1")
            elif len(stone) % 2 == 0:
                i = len(stone) // 2
                new.append(str(int(stone[:i])))
                new.append(str(int(stone[i:])))
            else:
                new.append(str(int(stone) * 2024))
        stones = new

    return len(stones)


def part2(data: str) -> int:
    stones = collections.Counter(data.split())
    for _ in range(75):
        new: collections.Counter[str] = collections.Counter()
        for stone, count in stones.items():
            if stone == "0":
                new["1"] += count
            elif len(stone) % 2 == 0:
                i = len(stone) // 2
                new[str(int(stone[:i]))] += count
                new[str(int(stone[i:]))] += count
            else:
                new[str(int(stone) * 2024)] += count
        stones = new

    return sum(stones.values())
