use std::collections::HashMap;
impl Solution {
  pub fn roman_to_int(s: String) -> i32 {
 let mut v = Vec::new();
 for char in s.chars() {
  v.push(char);
 }
 let mut maps = HashMap::new();
 maps.insert(String::from("I"),1);
 maps.insert(String::from("V"),5);
 maps.insert(String::from("X"),10);
 maps.insert(String::from("L"),50);
 maps.insert(String::from("C"),100);
 maps.insert(String::from("D"),500);
 maps.insert(String::from("M"),1000);
 let mut result = 0;
 for i in 0..s.len()  {
   let v1 = maps.get(&*(v[i].to_string())).unwrap().clone();
   let mut v2 = 0;
   if i < s.len()-1 {
     v2 = maps.get(&*(v[i+1].to_string())).unwrap().clone();
     if v1 < v2 {
      result -= v1;
     } else {
      result += v1;
     }
   } else {
     v2 = v1;
     result += v2;
   }
  }
 return result;
}
}
