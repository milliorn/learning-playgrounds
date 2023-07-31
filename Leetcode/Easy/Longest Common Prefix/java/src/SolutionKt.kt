class SolutionKtKt {
    fun longestCommonPrefix(strs: Array<String>): String {
        val prefix = StringBuilder(strs[0])
        var position = 0
        var maxCharIndex = strs[0].length
        for (i in 1 until strs.size) {
            maxCharIndex = Math.min(maxCharIndex, strs[i].length)
        }
        while (position < maxCharIndex) {
            val previousLetter = strs[0][position]
            for (i in 1 until strs.size) {
                if (previousLetter == strs[i][position]) {
                    continue
                }
                return prefix.substring(0, position)
            }
            ++position
            prefix.append(previousLetter)
        }
        return prefix.substring(0, position)
    }
}