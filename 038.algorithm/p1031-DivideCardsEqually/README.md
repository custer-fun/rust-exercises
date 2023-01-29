# [[NOIP2002 提高组] 均分纸牌](https://www.luogu.com.cn/problem/P1031)

## 题目描述

有$N$堆纸牌，编号分别为 $1,2,…,N$。每堆上有若干张，但纸牌总数必为 $N$ 的倍数。可以在任一堆上取若干张纸牌，然后移动。

移牌规则为：在编号为 $1$ 堆上取的纸牌，只能移到编号为 $2$ 的堆上；在编号为 $N$ 的堆上取的纸牌，只能移到编号为 $N-1$ 的堆上；其他堆上取的纸牌，可以移到相邻左边或右边的堆上。

现在要求找出一种移动方法，用最少的移动次数使每堆上纸牌数都一样多。

例如 $N=4$ 时，$4$ 堆纸牌数分别为 $9,8,17,6$。

移动 $3$ 次可达到目的：

- 从第三堆取 $4$ 张牌放到第四堆，此时每堆纸牌数分别为 $9,8,13,10$。
- 从第三堆取 $3$ 张牌放到第二堆，此时每堆纸牌数分别为 $9,11,10,10$。
- 从第二堆取 $1$ 张牌放到第一堆，此时每堆纸牌数分别为  $10,10,10,10$。

## 输入格式

第一行共一个整数 $N$，表示纸牌堆数。  
第二行共 $N$ 个整数 $A_1,A_2,\cdots,A_N$，表示每堆纸牌初始时的纸牌数。

## 输出格式

共一行，即所有堆均达到相等时的最少移动次数。

## 样例 #1

### 样例输入 #1

```
4
9 8 17 6
```

### 样例输出 #1

```
3
```

## 提示

对于 $100\%$ 的数据，$1  \le  N  \le  100$，$1 \le  A_i  \le 10000$。

**【题目来源】**

NOIP 2002 提高组第一题

## Rust
```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", solved(n, vec));
}

fn solved(n: usize, nums: Vec<usize>) -> usize {
    let mut result = 0;
    let s: usize = nums.iter().sum();
    let average = s / n;
    let mut diff = vec![];
    for &num in &nums {
        diff.push(num as i32 - average as i32);
    }

    for i in 0..diff.len() - 1 {
        if diff[i] != 0 {
            diff[i + 1] += diff[i];
            result += 1;
        }
    }
    result
}
```