package main
func isPalindrome(x int) bool {
	if x < 0 {
		return false;
	}
	sum := 0;
	n := x;
	for n != 0 {
		sum = sum *10 + n%10;
		n /= 10;
	}
	if sum == x {
		return true;
	}
	return false
}



