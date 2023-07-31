# @param {Integer[]} nums
# @return {Integer}
def third_max(nums)
    max1 = nil
    max2 = nil
    max3 = nil

    nums.each do |num|
      if num == max1 || num == max2 || num == max3
        next
      end

      if max1 == nil || num > max1
        max3 = max2
        max2 = max1
        max1 = num
      elsif max2 == nil || num > max2
        max3 = max2
        max2 = num
      elsif max3 == nil || num > max3
        max3 = num
      end
    end

    max3 == nil ? max1 : max3
end