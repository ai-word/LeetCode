var findMedianSortedArrays = function (nums1, nums2) {
    let nums_1 = nums1
    let nums_2 = nums2
    // 计算长度
    let len1 = nums_1.length
    let len2 = nums_2.length
    let arr2_c = 0
  // 边界判断
  if(len1==0) {
     if (len2 == 1) {
         arr2_c = nums_2[0]
     } else {
         let v = parseInt(len2/2)
         if(len2%2 == 1) {
             arr2_c = nums_2[v]
         } else {
             arr2_c = (nums_2[v-1] + nums_2[v])/2.0
         }
       return arr2_c
     }
   }
  if (len2 == 0) {
      if (len1 == 1) {
          arr2_c = nums_1[0]
      } else {
          let v = parseInt(len1/2)
          if(len1%2 == 1) {
              arr2_c = nums_1[v]
          } else {
              arr2_c = (nums_1[v-1] + nums_1[v])/2.0
          }
      }
      return arr2_c
  }
  let len_sum = len2 + len1
  let flag = 0
  let limit = 0
  if (len_sum%2==0) {
     flag = 2
      limit = len_sum / 2 + 1
  } else {
      flag = 1
      limit = len_sum / 2

  }
  let times = 0
    let ans = []
    while (times < limit) {
      let l1 = nums_1.length
      let l2 = nums_2.length
      if (l1 !==0 && l2!==0 ) {
          if (nums_1[0] > nums_2[0]){
              let v = nums_2.shift()
              ans.push(v)
          } else {
              let v = nums_1.shift()
              ans.push(v)
          }
      } else if (l1 == 0 && l2 !== 0){
          ans.push(nums_2.shift())
      } else if (l2 == 0 && l1 !== 0) {
          ans.push(nums_1.shift())
      }
        times ++
    }
  if (flag == 1) {
      return ans.pop()
  } else {
      return (ans.pop() + ans.pop())/2
  }
};

a = findMedianSortedArrays([2],[])
console.log(a)
