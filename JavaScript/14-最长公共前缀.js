var longestCommonPrefix = function (strs) {
  if (strs.length == 0) return "";
  if (strs.length == 1) return strs[0];
  var temp = strs[0];
  var res = "";
  for (let i = 0; i < temp.length; i++) {
    for (let j = 1; j < strs.length; j++) {
      if (temp[i] !== strs[j][i]) {
        return res;
      }
    }
    res += temp[i];
  }
  return res
}
