/**
 * @param {number} n
 * @return {string}
 */
var countAndSay = function(n) {
   if(n == 1) {
       return "1"
   } else {
       let str = countAndSay(n-1);
       let count = 1;
       let res = "";
       for (let i = 0;i < str.length;i++) {
           if (str[i] == str[i+1]) {
               count ++
           } else {
               res = res+count + str[i]
               count = 1
           }
       }
       return res
   }
};