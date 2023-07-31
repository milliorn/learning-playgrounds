func defangIPaddr(address string) string {
    var result string

		for _, c := range address {
			if c == '.' {
				result += "[.]"
			} else {
				result += string(c)
			}
		}

		return result
}