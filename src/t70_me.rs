pub fn climb_stairs(n: i32) -> i32 {
    let level = n as usize;
    let mut dp = vec![0; level + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..level + 1 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[level]
}
