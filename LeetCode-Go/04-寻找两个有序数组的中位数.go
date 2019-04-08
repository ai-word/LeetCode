func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	//边界判断
	nums_1 := nums1
	nums_2 := nums2
	len1 := len(nums1)
	len2 := len(nums2)
	if len1 == 0 {
		if len2 == 1 {
			return  float64(nums_2[0])
		} else {
			v := len2/2
			if len2 % 2 == 0{
				return  float64(nums_2[v-1]+nums_2[v])/2.0
			} else {
				return float64(nums_2[v])
			}
		}
	}
	if len2 == 0 {
		if len1 == 1 {
			return float64(nums_1[0])
		} else {
			v := len1/2
			if len1 % 2 == 0{
				return float64(nums_1[v-1]+nums_1[v]) / 2.0
			} else {
				return float64(nums_1[v])
			}
		}
	}
	len_sum := len1 + len2
	flag_s :=  0
	limit_c := 0
	if len_sum % 2 == 0 {
		flag_s = 2
		limit_c = len_sum / 2 + 1.0
	} else {
		flag_s = 1
		limit_c = len_sum / 2.0 + 1
	}
	times := 0
	ans := []int{}

	for;times < limit_c;{
		l1 := len(nums_1)
		l2 := len(nums_2)
		fmt.Println("1->",nums_1,"2->",nums_2)
		if l1 != 0 && l2 != 0{
			if nums_1[0] > nums_2[0]{
				ans = append(ans, nums_2[0])
				nums_2 = nums_2[1:]
			} else {
				ans = append(ans, nums_1[0])
				nums_1 = nums_1[1:]
			}
		} else if l1 == 0 && l2 != 0 {
			ans = append(ans, nums_2[0])
			nums_2 = nums_2[1:]
		} else if l2 == 0 && l1 != 0 {
			ans = append(ans, nums_1[0])
			nums_1 = nums_1[1:]
		}
		times += 1
	}
	if flag_s == 1 {
		l := len(ans)
		return float64(ans[l-1])
	} else {
		l := len(ans)
		return float64((ans[l-1] + ans[l-2]))/2
	}
}