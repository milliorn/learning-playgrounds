# @param {Integer[]} nums
# @return {Integer[]}
def single_number(nums)
  # Compute the XOR of all elements in the array
  xor = 0
  nums.each { |num| xor ^= num }
  
  # Find the rightmost set bit of the XOR result
  rightmost_set_bit = xor & -xor
  
  # Partition the numbers into two groups based on the rightmost set bit
  group1, group2 = 0, 0

  nums.each do |num|
      if num & rightmost_set_bit != 0
          group1 ^= num
      else
          group2 ^= num
      end
  end
  
  # Return the two numbers
  return [group1, group2]
end
