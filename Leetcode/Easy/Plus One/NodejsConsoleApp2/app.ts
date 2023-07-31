function plusOne(digits: number[]): number[] {
    let strToNum = BigInt(digits.join(""));
    ++strToNum;
    const numToStr = '' + strToNum;
    return numToStr.split("") as unknown as number[];
};

console.log(plusOne([1, 2, 3]))

setTimeout(function() {
    process.exit();
}, 5000);