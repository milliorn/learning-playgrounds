class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        maxs = nums[0]
        mins = 0
        sums = 0
        for num in nums:
            sums += num
            maxs = max(maxs, sums - mins)
            mins = min(mins, sums)

        return maxs


def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print_hi('PyCharm')

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
