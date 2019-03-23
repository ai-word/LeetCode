/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function(s) {
    let maps = {"I":1,"V":5,"X":10,"L":50,"C":100,"D":500,"M":1000};
    let res = 0
    for (let i = 0;i < s.length;i++ ){
        if ((i < s.length -1)&&(maps[s[i]] < maps[s[i + 1]])){
           res -= maps[s[i]]
        } else {
          res += maps[s[i]] 
        }
    }
    return res
};
