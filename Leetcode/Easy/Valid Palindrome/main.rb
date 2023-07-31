# @param {String} s
# @return {Boolean}
def is_palindrome(s)
    formatted_str = s.downcase.gsub(/[^a-z0-9]/, '')
    formatted_str == formatted_str.reverse
end