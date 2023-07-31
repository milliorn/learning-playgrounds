/**
 * @param {number[]} digits
 * @return {number[]}
 */

const plusOne = (digits) => {
    let strToNum = BigInt(digits.join(""));
    ++strToNum;
    const numToStr = '' + strToNum;
    return numToStr.split("");
};

console.log(plusOne([1, 2, 3]))

setTimeout(function () {
    process.exit();
}, 5000);