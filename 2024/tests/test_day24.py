from aoc2024.day24 import part1, part2

DATA1 = """
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
""".strip()

DATA2 = """
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
""".strip()

DATA3 = """
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 XOR y00 -> z00
x01 XOR y01 -> x01XORy01
x00 AND y00 -> x00ANDy00
x01XORy01 XOR x00ANDy00 -> z01
x02 XOR y02 -> x02XORy02
x01 AND y01 -> x01ANDy01
x01XORy01 AND x00ANDy00 -> x01XORy01ANDx00ANDy00
x01ANDy01 OR x01XORy01ANDx00ANDy00 -> x01ANDy01ORx01XORy01ANDx00ANDy00
x02XORy02 XOR x01ANDy01ORx01XORy01ANDx00ANDy00 -> z03
y02 AND x02 -> x02ANDy02
x02XORy02 AND x01ANDy01ORx01XORy01ANDx00ANDy00 -> x02XORy02ANDx01ANDy01ORx01XORy01ANDx00ANDy00
x02ANDy02 OR x02XORy02ANDx01ANDy01ORx01XORy01ANDx00ANDy00 -> z02
""".strip()


def test_part1() -> None:
    assert part1(DATA1) == 4
    assert part1(DATA2) == 2024


def test_part2() -> None:
    assert part2(DATA3) == "z02,z03"
