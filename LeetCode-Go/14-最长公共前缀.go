package main
func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	if len(strs) == 1 {
		return strs[0]
	}
	temp := strs[0]
	res := ""
	for i:=0;i<len(temp);i++ {
		for j:=1;j<len(strs);j++ {
			if i == len(strs[j]) || temp[i] != strs[j][i] {
				return  res
			}

		}
		res += string(temp[i]);
	}
	return res
}

