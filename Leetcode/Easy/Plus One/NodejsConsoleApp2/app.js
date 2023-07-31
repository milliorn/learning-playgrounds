function plusOne(digits) {
    let strToNum = BigInt(digits.join(""));
    ++strToNum;
    const numToStr = '' + strToNum;
    const result = numToStr.split("");
    return result;
}
;
console.log(plusOne([1, 2, 3]));
setTimeout(function () {
    process.exit();
}, 5000);
//# sourceMappingURL=app.js.map