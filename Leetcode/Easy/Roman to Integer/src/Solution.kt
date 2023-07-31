import java.util.HashMap

class Solution {
    fun romanToInt(s: String): Int {
        val map = roman()
        val length = s.length
        var result = map[s[length - 1]]!!
        for (i in length - 2 downTo 0) {
            result = getResult(s, map, result, i)
        }
        return result
    }

    private fun getResult(s: String, map: Map<Char, Int>, result: Int, i: Int): Int {
        var result = result
        val temp = s[i]
        if (map[temp]!! >= map[s[i + 1]]!!) {
            result += map[temp]!!
        } else {
            result -= map[temp]!!
        }
        return result
    }

    private fun roman(): Map<Char, Int> {
        val map: MutableMap<Char, Int> = HashMap()
        map['I'] = 1
        map['V'] = 5
        map['X'] = 10
        map['L'] = 50
        map['C'] = 100
        map['D'] = 500
        map['M'] = 1000
        return map
    }
}