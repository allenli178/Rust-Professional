const COINS: [u32; 8] = [1, 2, 5, 10, 20, 30, 50, 100];
pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut dp = vec![0; amount as usize + 1];
    dp[0] = 0;
    for i in 1..=amount as usize {
        // 找零数量加一
        dp[i] = dp[i - 1] + 1;
        for j in 1..COINS.len() {
            if COINS[j] <= i as u32 {
                dp[i] = dp[i].min(dp[i - COINS[j] as usize] + 1);
            }
        }
    }
    dp[amount as usize]
}
