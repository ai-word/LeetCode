var lengthOfLongestSubstring = function (s) {
  var sum = ""; // 存放的结果字符串
  var max = 0;
  var i, k = 0;
  while (i < s.length && k < s.length) {
    // 如果检测到sum中没有字符s[k]则sum+s[k],k++
    if (sum.indexOf(s[k]) === -1) {
      sum += s[k];
      k++;
    } else {  // 如果检测到sum中有字符s[k]则sum="",i++,k=i
      i++;
      k = i;
      sum = "";
    }
    max = Math.max(max, sum.length); // amx始终存储最大的
  }
  return max;
};
let res = lengthOfLongestSubstring("abcbdbdbdbd")
console.log(res)
