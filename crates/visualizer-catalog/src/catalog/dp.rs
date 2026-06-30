use crate::types::*;

pub fn dp_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry { algorithm: "fibonacci-dp".into(),
            meta: meta_simple!("fibonacci-dp","斐波那契 DP","Fibonacci DP",Dp,"自底向上递推，避免递归重复计算。",time="O(n)",space="O(n)",use=["DP 入门"]),
            samples: CodeSamples {
                cpp: cpp!("int fibonacciDP(int n) {","  if (n <= 1) return n;","  vector<int> dp(n+1);","  dp[0]=0; dp[1]=1;","  for (int i=2; i<=n; i++)","    dp[i]=dp[i-1]+dp[i-2];","  return dp[n];","}"),
                python: py!("def fibonacci_dp(n):","    if n <= 1: return n","    dp = [0] * (n+1)","    dp[1] = 1","    for i in range(2, n+1):","        dp[i] = dp[i-1] + dp[i-2]","    return dp[n]"),
                rust: rs!("fn fibonacci_dp(n: usize) -> usize {","    if n <= 1 { return n; }","    let mut dp = vec![0; n+1];","    dp[1] = 1;","    for i in 2..=n {","        dp[i] = dp[i-1] + dp[i-2];","    }","    dp[n]","}"),
            },
            line_map: lm!("fib-dp"=>[1],"init"=>[3],"compute"=>[5],"done"=>[7]),
        },
        AlgorithmEntry { algorithm: "knapsack".into(),
            meta: meta_simple!("knapsack","0/1 背包","Knapsack",Dp,"选择物品使总价值最大且不超容量。",time="O(n·W)",space="O(n·W)",use=["资源分配","投资组合"]),
            samples: CodeSamples {
                cpp: cpp!("int knapsack(vector<int>& w,vector<int>& v,int W){","  int n=w.size();","  vector<vector<int>> dp(n+1,vector<int>(W+1));","  for(int i=1;i<=n;i++)","    for(int j=0;j<=W;j++){","      if(w[i-1]<=j)","        dp[i][j]=max(dp[i-1][j],dp[i-1][j-w[i-1]]+v[i-1]);","      else dp[i][j]=dp[i-1][j];","    }","  return dp[n][W];","}"),
                python: py!("def knapsack(w, v, W):","    n = len(w)","    dp = [[0]*(W+1) for _ in range(n+1)]","    for i in range(1, n+1):","        for j in range(W+1):","            if w[i-1] <= j:","                dp[i][j]=max(dp[i-1][j],dp[i-1][j-w[i-1]]+v[i-1])","            else: dp[i][j]=dp[i-1][j]","    return dp[n][W]"),
                rust: rs!("fn knapsack(w: &[usize], v: &[i32], cap: usize) -> i32 {","    let n = w.len();","    let mut dp = vec![vec![0; cap+1]; n+1];","    for i in 1..=n {","        for j in 0..=cap {","            if w[i-1] <= j {","                dp[i][j]=dp[i-1][j].max(dp[i-1][j-w[i-1]]+v[i-1]);","            } else { dp[i][j]=dp[i-1][j]; }","        }","    }","    dp[n][cap]","}"),
            },
            line_map: lm!("knapsack"=>[1],"dp-update"=>[6],"done"=>[11]),
        },
        AlgorithmEntry { algorithm: "lcs".into(),
            meta: meta_simple!("lcs","最长公共子序列","LCS",Dp,"求两个序列的最长公共子序列长度。",time="O(m·n)",space="O(m·n)",use=["文本对比","版本控制"]),
            samples: CodeSamples {
                cpp: cpp!("int lcs(string& a, string& b) {","  int m=a.size(), n=b.size();","  vector<vector<int>> dp(m+1,vector<int>(n+1));","  for(int i=1;i<=m;i++)","    for(int j=1;j<=n;j++)","      if(a[i-1]==b[j-1]) dp[i][j]=dp[i-1][j-1]+1;","      else dp[i][j]=max(dp[i-1][j],dp[i][j-1]);","  return dp[m][n];","}"),
                python: py!("def lcs(a, b):","    m, n = len(a), len(b)","    dp = [[0]*(n+1) for _ in range(m+1)]","    for i in range(1, m+1):","        for j in range(1, n+1):","            if a[i-1]==b[j-1]: dp[i][j]=dp[i-1][j-1]+1","            else: dp[i][j]=max(dp[i-1][j],dp[i][j-1])","    return dp[m][n]"),
                rust: rs!("fn lcs(a: &[u8], b: &[u8]) -> usize {","    let (m,n) = (a.len(), b.len());","    let mut dp = vec![vec![0; n+1]; m+1];","    for i in 1..=m { for j in 1..=n {","        if a[i-1]==b[j-1] { dp[i][j]=dp[i-1][j-1]+1; }","        else { dp[i][j]=dp[i-1][j].max(dp[i][j-1]); }","    }}","    dp[m][n]","}"),
            },
            line_map: lm!("lcs"=>[1],"dp-cell"=>[6],"done"=>[8]),
        },
        AlgorithmEntry { algorithm: "lis".into(),
            meta: meta_simple!("lis","最长递增子序列","LIS",Dp,"求数组中最长严格递增子序列的长度。",time="O(n²)",space="O(n)",use=["序列分析","套娃问题"]),
            samples: CodeSamples {
                cpp: cpp!("int lis(vector<int>& arr) {","  int n=arr.size(), ans=1;","  vector<int> dp(n,1);","  for(int i=0;i<n;i++)","    for(int j=0;j<i;j++)","      if(arr[j]<arr[i])","        dp[i]=max(dp[i],dp[j]+1);","  return *max_element(dp.begin(),dp.end());","}"),
                python: py!("def lis(arr):","    n = len(arr)","    dp = [1] * n","    for i in range(n):","        for j in range(i):","            if arr[j] < arr[i]:","                dp[i] = max(dp[i], dp[j] + 1)","    return max(dp)"),
                rust: rs!("fn lis(arr: &[i32]) -> usize {","    let n = arr.len();","    let mut dp = vec![1; n];","    for i in 0..n { for j in 0..i {","        if arr[j] < arr[i] {","            dp[i] = dp[i].max(dp[j] + 1);","        }","    }}","    dp.into_iter().max().unwrap_or(0)","}"),
            },
            line_map: lm!("lis"=>[1],"init"=>[3],"compare"=>[5],"update"=>[6],"done"=>[9]),
        },
    ]
}
