import java.util.HashMap;
import java.util.Map;

public class Solution {

  public int romanToInt(String s) {
    Map<Character, Integer> map = roman();
    int length = s.length();
    int result = map.get(s.charAt(length - 1));

    for (int i = length - 2; i >= 0; i--) {
      result = getResult(s, map, result, i);
    }

    return result;
  }

  private int getResult(String s, Map<Character, Integer> map, int result, int i) {
    char temp = s.charAt(i);
    if (map.get(temp) >= map.get(s.charAt(i + 1))) {
      result += map.get(temp);
    } else {
      result -= map.get(temp);
    }
    return result;
  }

  private Map<Character, Integer> roman() {
    Map<Character, Integer> map = new HashMap<>();
    map.put('I', 1);
    map.put('V', 5);
    map.put('X', 10);
    map.put('L', 50);
    map.put('C', 100);
    map.put('D', 500);
    map.put('M', 1000);
    return map;
  }
}
