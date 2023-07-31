/**
 * @param {string} address
 * @return {string}
 */
function defangIPaddr(address) {
  return address.replaceAll(".", "[.]");
}

let testStr = defangIPaddr("1.1.1.1");

console.log(testStr);

let address = "1[.]1[.]1[.]1"

console.log(testStr === address);