package main

func isValid(s string) bool {
	stack := make([]byte, 0)
	for _, v := range s {
		if v == '[' || v == '(' || v == '{' {
			stack = append(stack, byte(v))
		} else {
			if (len(stack) != 0 && match(stack[len(stack)-1], byte(v))) {
				stack = pop(stack)
				continue
			} else {
				return false
			}
		}
	}

	if (len(stack) == 0) {
		return true
	} else {
		return false
	}
}
func match(left, right byte) bool {
	switch right {
	case ']':
		if left != '[' {
			return false
		}
	case ')':
		if left != '(' {
			return false
		}
	case '}':
		if left != '{' {
			return false
		}
	}
	return true
}

func pop(stack []byte) []byte {
	return stack[:len(stack)-1]
}
