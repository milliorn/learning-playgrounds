class Solution {

/**
 * @param String $address
 * @return String
 */
function defangIPaddr(string $address): string {
  return str_replace(".", "[.]", $address);
}

}