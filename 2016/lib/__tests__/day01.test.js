"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const day01_1 = require("../day01");
describe('day01', () => {
    it('runs part1', () => {
        expect(day01_1.part1('R2, L3')).toBe(5);
        expect(day01_1.part1('R2, R2, R2')).toBe(2);
        expect(day01_1.part1('R5, L5, R5, R3')).toBe(12);
    });
    it('runs part2', () => {
        expect(day01_1.part2('R8, R4, R4, R8')).toBe(4);
    });
});
