use crate::types::*;

pub fn recursion_entries() -> Vec<AlgorithmEntry> {
    vec![
        AlgorithmEntry {
            algorithm: "factorial".into(),
            meta: meta_simple!("factorial","阶乘","Factorial",Recursive,"n! = n × (n-1) × ... × 1。",time="O(n)",space="O(n)",use=["递归入门"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "int factorial(int n) {",
                    "  if (n <= 1) return 1;",
                    "  int result = 1;",
                    "  for (int i = 2; i <= n; i++)",
                    "    result *= i;",
                    "  return result;",
                    "}"
                ),
                python: py!(
                    "def factorial(n):",
                    "    if n <= 1:",
                    "        return 1",
                    "    result = 1",
                    "    for i in range(2, n + 1):",
                    "        result *= i",
                    "    return result"
                ),
                rust: rs!(
                    "fn factorial(n: u64) -> u64 {",
                    "    if n <= 1 { return 1; }",
                    "    let mut result = 1;",
                    "    for i in 2..=n {",
                    "        result *= i;",
                    "    }",
                    "    result",
                    "}"
                ),
            },
            line_map: lm!("factorial"=>[1],"multiply"=>[5],"done"=>[7]),
        },
        AlgorithmEntry {
            algorithm: "fibonacci".into(),
            meta: meta_simple!("fibonacci","斐波那契","Fibonacci",Recursive,"F(n) = F(n-1) + F(n-2)。",time="O(n)",space="O(n)",use=["递归与动态规划"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "int fibonacci(int n) {",
                    "  if (n <= 1) return n;",
                    "  vector<int> dp(n + 1);",
                    "  dp[0] = 0; dp[1] = 1;",
                    "  for (int i = 2; i <= n; i++)",
                    "    dp[i] = dp[i - 1] + dp[i - 2];",
                    "  return dp[n];",
                    "}"
                ),
                python: py!(
                    "def fibonacci(n):",
                    "    if n <= 1:",
                    "        return n",
                    "    dp = [0] * (n + 1)",
                    "    dp[1] = 1",
                    "    for i in range(2, n + 1):",
                    "        dp[i] = dp[i - 1] + dp[i - 2]",
                    "    return dp[n]"
                ),
                rust: rs!(
                    "fn fibonacci(n: usize) -> usize {",
                    "    if n <= 1 { return n; }",
                    "    let mut dp = vec![0; n + 1];",
                    "    dp[1] = 1;",
                    "    for i in 2..=n {",
                    "        dp[i] = dp[i - 1] + dp[i - 2];",
                    "    }",
                    "    dp[n]",
                    "}"
                ),
            },
            line_map: lm!("fibonacci"=>[1],"compute"=>[6],"done"=>[8]),
        },
        AlgorithmEntry {
            algorithm: "tower-of-hanoi".into(),
            meta: meta_simple!("tower-of-hanoi","汉诺塔","Tower of Hanoi",Recursive,"经典递归问题，将盘子从 A 柱移到 C 柱。",time="O(2ⁿ)",space="O(n)",use=["递归思维训练"]),
            samples: CodeSamples {
                cpp: cpp!(
                    "void hanoi(int n, char from, char to, char aux) {",
                    "  if (n == 0) return;",
                    "  hanoi(n - 1, from, aux, to);",
                    "  printf(\"Move disk %d from %c to %c\\n\",n,from,to);",
                    "  hanoi(n - 1, aux, to, from);",
                    "}"
                ),
                python: py!(
                    "def hanoi(n, from_peg, to_peg, aux_peg):",
                    "    if n == 0:",
                    "        return",
                    "    hanoi(n - 1, from_peg, aux_peg, to_peg)",
                    "    print(f\"Move disk {n} from {from_peg} to {to_peg}\")",
                    "    hanoi(n - 1, aux_peg, to_peg, from_peg)"
                ),
                rust: rs!(
                    "fn hanoi(n: usize, from: char, to: char, aux: char) {",
                    "    if n == 0 { return; }",
                    "    hanoi(n - 1, from, aux, to);",
                    "    println!(\"Move disk {n} from {from} to {to}\");",
                    "    hanoi(n - 1, aux, to, from);",
                    "}"
                ),
            },
            line_map: lm!("hanoi"=>[1],"move"=>[4],"done"=>[6]),
        },
    ]
}
