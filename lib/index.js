"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = __importDefault(require("fs"));
const day01 = __importStar(require("./day01"));
const day02 = __importStar(require("./day02"));
const days = {
    '01': day01,
    '02': day02
};
function run(name) {
    const day = days[name];
    const data = fs_1.default.readFileSync(`data/day${name}.txt`, 'utf-8').trim();
    console.log(`day ${name}`);
    console.log(`  part 1: ${day.part1(data)}`);
    console.log(`  part 2: ${day.part2(data)}`);
}
if (process.argv.length == 3) {
    run(process.argv[2]);
}
else {
    for (const day in days) {
        run(day);
    }
}
