class Solution {
    func defangIPaddr(_ address: String) -> String {
        var result = ""

        for char in address {
          if char == "."{
            result += "[.]"
          } else {
            result += String(char)
          }
        }

        return result
    }
}