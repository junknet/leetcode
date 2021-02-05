class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        ans = ""
        dp = [[False]*n for _ in range(n)]
        for l in range(n):
            for start in range(n):
                end = start+l
                if end >= n:
                    break
                if l == 0:
                    dp[start][end] = True
                elif l == 1:
                    dp[start][end] = (s[start] == s[end])
                else:
                    dp[start][end] = (s[start] == s[end]
                                      and dp[start+1][end-1])
                if dp[start][end] and l >= len(ans):
                    ans = s[start:end+1]
        return ans


a = Solution().longestPalindrome("aba")
print(a)
