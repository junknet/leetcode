pub fn is_match(s: String, p: String) -> bool {
    let p_chars: Vec<char> = p.chars().collect();
    let s_chars: Vec<char> = s.chars().collect();
    let (m, n) = (s.len(), p.len());
    let mut dp: Vec<Vec<bool>> =
        vec![vec![false; p_chars.len() + 1]; s_chars.len() + 1];
    dp[0][0] = true;
    let matches = |i: usize, j: usize| -> bool {
        if i == 0 {
            return false;
        };
        if p_chars[j - 1] == '.' {
            return true;
        };
        s_chars[i - 1] == p_chars[j - 1]
    };

    for i in 0..m + 1 {
        for j in 1..n + 1 {
            if p_chars[j - 1] == '*' {
                dp[i][j] |= dp[i][j - 2];
                if matches(i, j - 1) {
                    dp[i][j] |= dp[i - 1][j];
                }
            } else {
                if matches(i, j) {
                    dp[i][j] |= dp[i - 1][j - 1];
                }
            }
        }
    }
    dp[m][n]
}
