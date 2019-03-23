use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec!["1".to_string()];
        let mut map = HashMap::new();
        map.insert(String::from("{"),"}");
        map.insert(String::from("["),"]");
        map.insert(String::from("("),")");
        for it in s.chars() {
            let c2s = it.to_string();
            let ss = map.get(&c2s);
            if let Some(b) = ss{
                stack.push(c2s.clone());
            } else {
                let mut s1 = stack.pop().unwrap();
                let s2 = map.get(&s1);
                if let Some(U) = s2{
                    if *U != c2s {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        if stack.len()==1{
            return true;
        } else {
            return false;
        }
    }
}
