function thirdMax(nums: number[]): number {
    const min = Number.MIN_SAFE_INTEGER;
    let max1 = min;
    let max2 = min;
    let max3 = min;
    let i = 0;

    while (i < nums.length) {
        const element = nums[i];
        if (element === max1 || element === max2 || element === max3) {
            i++;
            continue;
        }

        if (element > max1) {
            max3 = max2;
            max2 = max1;
            max1 = element;
        } else if (element > max2) {
            max3 = max2;
            max2 = element;
        } else if (element > max3) {
            max3 = element;
        }

        i++;
    }

    return max3 !== min ? max3 : max1;
};