"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const day02_1 = require("../day02");
const data = ['ULL', 'RRDDD', 'LURDL', 'UUUUD'].join('\n');
describe('day02', () => {
    it('runs part1', () => {
        expect(day02_1.part1(data)).toBe('1985');
    });
    it('runs part2', () => {
        expect(day02_1.part2(data)).toBe('5DB3');
    });
});
