import collections
from dataclasses import dataclass


@dataclass
class File:
    id_: int
    used: int
    free: int


def part1(data: str) -> int:
    files = collections.deque(parse(data))

    # compress the files
    free = 0
    block = 0
    check = 0
    while files:
        # try to place the next file from the right
        rgt = files.pop()
        while rgt.used > 0:
            # if we are out of free space, copy the next file from the left
            if free == 0 and files:
                lft = files.popleft()
                for _ in range(lft.used):
                    check += block * lft.id_
                    block += 1
                free = lft.free

            # if we now have some free space, fill it from the right
            fill = min(rgt.used, free) if files else rgt.used
            for _ in range(fill):
                check += block * rgt.id_
                block += 1
            rgt.used -= fill
            free -= fill

    return check


def part2(data: str) -> int:
    files = parse(data)

    # compress the files
    block = 0
    check = 0
    free = 0
    i = 0
    while i < len(files):
        # first, checksum the current file from the left (or 0s if it was moved)
        for _ in range(files[i].used):
            check += block * files[i].id_
            block += 1
        free = files[i].free

        # search for files from the right that can fill the left's free space
        while True:
            for j in range(len(files) - 1, i, -1):
                if files[j].id_ > 0 and 0 < files[j].used <= free:
                    # move the file from the right
                    for _ in range(files[j].used):
                        check += block * files[j].id_
                        block += 1
                    free -= files[j].used
                    files[j].id_ = 0

                    # optimization: remove any moved files from the right side
                    # since they would only contribute 0s to the checksum
                    while files[-1].id_ == 0:
                        files.pop()
                    break
            else:
                # nothing was found that could move, so move on from the left
                break

        # skip any remaining free space
        block += free
        i += 1

    return check


def parse(data: str) -> list[File]:
    if len(data) % 2 == 1:
        data += "0"
    return [
        File(id_=id_, used=int(u), free=int(f))
        for id_, (u, f) in enumerate(zip(data[0::2], data[1::2]))
    ]
