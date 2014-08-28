#![feature(macro_rules)]

extern crate num;

use std::num::pow;
use std::num::Zero;
use num::rational::Rational;
use num::rational::Ratio;

macro_rules! try_opt(
    ($e:expr) => (match $e { None => return None, Some(e) => e })
)


// TODO: Comments

#[deriving(Clone)]
enum Tree {
    Node(Option<Box<Tree>>, Option<Box<Tree>>)
}

fn main() {
    let nums = vec!(8i, 8, 3, 3);
    let ops = vec!("+", "-", "/", "*");
    let goal = 24i;
    solve(goal, nums.as_slice(), ops.as_slice());
}

fn solve(goal: int, nums: &[int], ops: &[&str]) {
    let trees: Vec<Box<Tree>> = gen_trees(nums.len()).move_iter().map(|t| box t).collect();
    // TODO: Write own permutation alg to avoid vec copies
    for nums_perm in nums.permutations() {
        let mut range_end = 0;
        for i in range(0, nums.len() - 1) {
            range_end += pow(ops.len(), i)
        }
        range_end *= nums.len() - 1;
        for i in range(0, range_end) {
            let mut digits = to_base(i, ops.len());
            while digits.len() < nums.len() - 1 {
                digits.push(0);
            }
            let ops_perm: Vec<&str> = digits.iter().map(|d| ops[*d]).collect();
            for tree in trees.iter() {
                let (eq, res) = match dfs(tree, nums_perm.as_slice(), ops_perm.as_slice()) {
                    None => continue,
                    Some((eq, res)) => (eq, res)
                };
                if res == Ratio::from_integer(goal) {
                    println!("{} = {}", eq, res);
                }
            }
        }
    }
}

fn to_base(mut n: uint, b: uint) -> Vec<uint> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % b);
        n = n / b;
    }
    digits.reverse();
    digits
}

fn dfs(tree: &Box<Tree>, nums: &[int], ops: &[&str]) -> Option<(String, Rational)> {
    fn helper(tree: &Box<Tree>, nums: &[int], ops: &[&str], num_leaves: &mut uint,
              num_ops: &mut uint) -> Option<(String, Rational)> {
        match tree {
            &box Node(None, None) => {
                let res = nums[*num_leaves];
                *num_leaves += 1;
                Some((res.to_string(), Ratio::from_integer(res)))
            },
            &box Node(Some(ref left), Some(ref right)) => {
                let (l, l_res) = try_opt!(helper(left, nums, ops, num_leaves, num_ops));
                let (r, r_res) = try_opt!(helper(right, nums, ops, num_leaves, num_ops));
                let eq = format!("({} {} {})", l, ops[*num_ops], r);
                let res = match ops[*num_ops] {
                    "+" => l_res + r_res,
                    "-" => l_res - r_res,
                    "*" => l_res * r_res,
                    "/" => if r_res.is_zero() {
                        return None;
                    } else {
                        l_res / r_res
                    },
                    op  => fail!("Unknown operation: {}", op)
                };
                *num_ops += 1;
                Some((eq, res))
            }
            &box Node(_, _) => fail!("Binary tree should be full!")
        }
    }

    helper(tree, nums, ops, &mut 0, &mut 0)
}

fn gen_trees(size: uint) -> Vec<Tree> {
    let mut dp = Vec::new();
    dp.push(vec!(Node(None, None)));
    for i in range(1, size) {
        let mut v = Vec::new();
        for j in range(0, i) {
            for left in dp[j].iter() {
                for right in dp[i - j - 1].iter() {
                    v.push(Node(Some(box left.clone()), Some(box right.clone())));
                }
            }
        }
        dp.push(v);
    }

    dp.pop().unwrap()
}
