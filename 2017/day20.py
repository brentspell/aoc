import re

import numpy as np


def part1(data):
    # parse into a [particle, moment, coordinate] tensor
    particles = np.stack([parse(s) for s in data.split("\n")])
    # the particle with the smallest acceleration
    # will ultimately remain closest to the origin
    return np.argmin(norm(particles[:, 2]))


def part2(data):
    # parse into a [particle, moment, coordinate] tensor
    new = np.stack([parse(s) for s in data.split("\n")])
    done = False
    while not done and len(new) > 1:
        # update velocity/acceleration this tick
        old = new.copy()
        new[:, 1] += new[:, 2]
        new[:, 0] += new[:, 1]
        # continue while any are still moving toward each other
        old_dist = distance(old[:, 0][np.newaxis], old[:, 0][:, np.newaxis])
        new_dist = distance(new[:, 0][np.newaxis], new[:, 0][:, np.newaxis])
        done = not np.any(new_dist < old_dist)
        # destroy any new that collided this tick
        dup = np.all(new[:, 0][np.newaxis] == new[:, 0][:, np.newaxis], axis=-1)
        np.fill_diagonal(dup, False)
        new = np.delete(new, np.any(dup, axis=-1), axis=0)
    return len(new)


def parse(line):
    match = re.match(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, "
        r"v=<(-?\d+),(-?\d+),(-?\d+)>, "
        r"a=<(-?\d+),(-?\d+),(-?\d+)>",
        line,
    )
    return np.split(np.array([int(match.group(i + 1)) for i in range(9)]), 3)


def distance(x, y):
    return norm(x - y)


def norm(x):
    return np.linalg.norm(x, 2, axis=-1)
