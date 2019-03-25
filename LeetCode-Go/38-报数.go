func countAndSay(n int) string {
	if n == 1 {
		return "1"
	} else {
		str := countAndSay(n-1)
		count := 1
		res := ""
		for i:=0;i<len(str);i++ {
			if i+1<len(str)&&str[i]==str[i+1]{
				count ++
			} else {
			   res = res + strconv.Itoa(count)+string(str[i])
			   // 重置
			   count = 1
			}
		}
		return res
	}
}