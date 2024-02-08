#![allow(non_snake_case, unused_macros)]

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::prelude::*;
use std::vec;
use svg::node::{
    element::{Circle, Group, Line, Rectangle, Style, Title},
    Text,
};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

const DIR: &str = "RDLU";
const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

fn can_move(N: usize, h: &Vec<Vec<char>>, v: &Vec<Vec<char>>, i: usize, j: usize, dir: usize) -> bool {
    let (di, dj) = DIJ[dir];
    let i2 = i + di;
    let j2 = j + dj;
    if i2 >= N || j2 >= N {
        return false;
    }
    if di == 0 {
        v[i][j.min(j2)] == '0'
    } else {
        h[i.min(i2)][j] == '0'
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub N: usize,
    pub h: Vec<Vec<char>>,
    pub v: Vec<Vec<char>>,
    pub d: Vec<Vec<i64>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.N)?;
        for i in 0..self.N - 1 {
            writeln!(f, "{}", self.h[i].iter().collect::<String>())?;
        }
        for i in 0..self.N {
            writeln!(f, "{}", self.v[i].iter().collect::<String>())?;
        }
        for i in 0..self.N {
            writeln!(f, "{}", self.d[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize,
        h: [Chars; N - 1],
        v: [Chars; N],
        d: [[i64; N]; N]
    }
    Input { N, h, v, d }
}

pub struct Output {
    pub out: Vec<char>,
}

pub fn parse_output(_input: &Input, f: &str) -> Result<Output, String> {
    let f = f.trim();
    if f.len() > 100000 {
        return Err(format!("Too long route: {}", f.len()));
    }
    Ok(Output {
        out: f.chars().collect(),
    })
}

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let N = rng.gen_range(20i32..=40) as usize;
    let w = rng.gen_range(1..=N as i32);
    let c = rng.gen_range(1..=N as i32 / 2);
    let (h, v) = loop {
        let mut h = mat!['0'; N - 1; N];
        let mut v = mat!['0'; N; N - 1];
        for _ in 0..w {
            let dir = rng.gen_range(0..4);
            if dir <= 1 {
                let i = rng.gen_range(0..N as i32 - 1) as usize;
                let j = rng.gen_range(0..N as i32) as usize;
                let k = rng.gen_range(3..=N as i32 / 2) as usize;
                for p in 0..k {
                    let j2 = if dir == 0 { j + p } else { j - p };
                    if j2 >= N {
                        break;
                    }
                    h[i][j2] = '1';
                }
            } else {
                let i = rng.gen_range(0..N as i32) as usize;
                let j = rng.gen_range(0..N as i32 - 1) as usize;
                let k = rng.gen_range(3..=N as i32 / 2) as usize;
                for p in 0..k {
                    let i2 = if dir == 0 { i + p } else { i - p };
                    if i2 >= N {
                        break;
                    }
                    v[i2][j] = '1';
                }
            }
        }
        let mut visited = mat![false; N; N];
        let mut stack = vec![(0, 0)];
        visited[0][0] = true;
        let mut count = 0;
        while let Some((i, j)) = stack.pop() {
            count += 1;
            for dir in 0..4 {
                if can_move(N, &h, &v, i, j, dir) {
                    let (di, dj) = DIJ[dir];
                    let i2 = i + di;
                    let j2 = j + dj;
                    if visited[i2][j2].setmax(true) {
                        stack.push((i2, j2));
                    }
                }
            }
        }
        if count == N * N {
            break (h, v);
        }
    };
    let mut d0 = mat![0.0; N; N];
    let mut d = mat![0; N; N];
    let mut chosen = mat![0; N; N];
    for iter in 1..=c {
        let i = rng.gen_range(0..N as i32) as usize;
        let j = rng.gen_range(0..N as i32) as usize;
        let m = rng.gen_range(N as i32..=(N * N) as i32 / c);
        let b = rng.gen_range(0.0..2.0);
        let mut list = vec![(i, j)];
        chosen[i][j] = iter;
        d0[i][j] = b;
        for _ in 1..m {
            loop {
                let &(i, j) = list.choose(&mut rng).unwrap();
                let dir = rng.gen_range(0..4i32) as usize;
                if can_move(N, &h, &v, i, j, dir) {
                    let i2 = i + DIJ[dir].0;
                    let j2 = j + DIJ[dir].1;
                    if chosen[i2][j2].setmax(iter) {
                        list.push((i2, j2));
                        d0[i2][j2] = b;
                        break;
                    }
                }
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            d[i][j] = f64::powf(10.0, d0[i][j] + rng.gen_range(0.0..1.0)).round() as i64;
        }
    }
    Input { N, h, v, d }
}
