# @param {Integer[]} nums
# @return {Integer}
def single_number(nums)
    answer = 0
    nums.each { |i| answer ^= i }
    answer
end