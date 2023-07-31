function singleNumber(nums: number[]): number {
    let answer: number = 0;
    nums.forEach(e => answer = answer ^ e);
    return answer;
};
