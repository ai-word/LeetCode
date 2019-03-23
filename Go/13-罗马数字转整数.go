package main
func romanToInt(s string) int {
	maps := make(map[string]int)
	maps["I"] = 1
	maps["V"] = 5
	maps["X"] = 10
	maps["L"] = 50
	maps["C"] = 100
	maps["D"] = 500
	maps["M"] = 1000
	res := int(0)
	for i := 0; i< len(s);i++ {
		if i < len(s)-1 && maps[string(s[i])] < maps[string(s[i+1])] {
			res -= maps[string(s[i])]
		} else {
			res += maps[string(s[i])]
		}
	}
	return res
}

