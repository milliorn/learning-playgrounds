# @param {Integer} num
# @return {Boolean}
def check_perfect_number(num)
  return false if num <= 1

  divisor_sum = 1
  divisor = 2

  while divisor * divisor <= num
    if num % divisor == 0
      divisor_sum += divisor
      divisor_sum += (num / divisor) if divisor != num / divisor
    end
    divisor += 1
  end

  divisor_sum == num
end