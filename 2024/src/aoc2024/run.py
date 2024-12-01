import argparse
import importlib
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser("aoc-run")
    parser.add_argument(
        "problems",
        type=str,
        nargs="*",
        help="names of problems to run",
    )
    args = parser.parse_args()

    problems = [f"{int(p):02d}" for p in args.problems or range(1, 26)]
    for problem in problems:
        try:
            module = importlib.import_module(f"aoc2024.day{problem}")
        except Exception:
            if args.problems:
                raise
            continue
        data = (Path(__file__).parents[2] / "data" / f"day{problem}.txt").read_text()
        print(f"Problem {problem}")
        print("  part 1:", module.part1(data))
        if problem != 25:
            print("  part 2:", module.part2(data))
            print()


if __name__ == "__main__":
    main()
