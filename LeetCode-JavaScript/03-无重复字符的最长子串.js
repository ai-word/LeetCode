/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
  var sum = "";
  var max = 0;
  var i = 0;
  var k = i;
  while (i < s.length && k < s.length) {
    if (sum.indexOf(s[k]) === -1) {
      sum += s[k];
      k++;
    } else {
      i++;
      k = i;
      sum = "";
    }
    max = Math.max(max, sum.length);
  }
  return max;
};
let res = lengthOfLongestSubstring("abcag")
console.log(res)
