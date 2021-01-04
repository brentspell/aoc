import re
from collections import defaultdict

START_REGEX = r"Begin in state (.)."
STEPS_REGEX = r"Perform a diagnostic checksum after (\d+) steps."
PREV_STATE_REGEX = r"In state (.):"
PREV_VALUE_REGEX = r"  If the current value is (\d+):"
NEXT_VALUE_REGEX = r"    - Write the value (\d+)."
NEXT_STEP_REGEX = r"    - Move one slot to the (left|right)."
NEXT_STATE_REGEX = r"    - Continue with state (.)."


def part1(data):
    lines = data.split("\n")
    state = re.match(START_REGEX, lines[0]).group(1)
    steps = int(re.match(STEPS_REGEX, lines[1]).group(1))
    trans = {}
    for i in range(3, len(lines), 10):
        prev_state = re.match(PREV_STATE_REGEX, lines[i]).group(1)
        for j in range(i + 1, i + 8, 4):
            prev_value = int(re.match(PREV_VALUE_REGEX, lines[j]).group(1))
            next_value = int(re.match(NEXT_VALUE_REGEX, lines[j + 1]).group(1))
            next_step = re.match(NEXT_STEP_REGEX, lines[j + 2]).group(1)
            next_state = re.match(NEXT_STATE_REGEX, lines[j + 3]).group(1)
            trans[(prev_state, prev_value)] = (next_value, next_step, next_state)

    tape = defaultdict(int)
    position = 0
    for _ in range(steps):
        next_value, next_step, next_state = trans[(state, tape[position])]
        tape[position] = next_value
        position += -1 if next_step == "left" else 1
        state = next_state

    return sum(tape.values())


def part2(_data):
    return 0
