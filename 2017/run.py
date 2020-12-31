import os
import sys
from importlib import import_module


def run(day):
    mod = import_module(f"day{day:02d}")
    with open(os.path.join("data", f"day{day:02d}.txt"), "r") as file:
        data = file.read().strip()

    print(f"day {day:02d}")
    print(f"    part 1: {mod.part1(data)}")
    print(f"    part 2: {mod.part2(data)}")


if __name__ == "__main__":
    if len(sys.argv) == 2:
        run(int(sys.argv[1]))
    else:
        for i in range(25):
            try:
                run(i + 1)
            except ModuleNotFoundError:
                pass
