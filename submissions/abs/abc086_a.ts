import * as fs from "fs";
const inputs = fs.readFileSync("/dev/stdin", "utf8").split(" ");
const a = +inputs[0];
const b = +inputs[1];
if ((a * b) % 2 === 1) {
    console.log("Odd");
}
else {
    console.log("Even");
}