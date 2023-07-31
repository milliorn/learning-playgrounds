class Solution {
    func singleNumber(_ nums: [Int]) -> Int {
        var answer = 0;

        for i in nums {
          answer ^= i;
        }

        return answer;
    }
}