function checkPerfectNumber(num: number): boolean {
    if (num <= 1) return false;

    let divisorSum = 1;

    for (let divisor = 2; divisor * divisor <= num; divisor++) {
        if (num % divisor === 0) {
            divisorSum += divisor;
            if (divisor != num / divisor) divisorSum += num / divisor | 0;
        }
    }

    return divisorSum === num;
};