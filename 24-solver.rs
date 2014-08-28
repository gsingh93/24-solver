#![feature(macro_rules)]

extern crate num;

use std::num::{pow, Zero};
use num::rational::{Ratio, Rational};

// Same as try!, but for Options
macro_rules! try_opt(
    ($e:expr) => (match $e { None => return None, Some(e) => e })
)

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

// Print all equations which use each num from `nums` once, use only
// the operators in `ops`, and have a result of `goal`. The algorithm
// is as follows:
//
//   1. Generate all trees that have `nums.len()` leaf nodes. This
//      corresponds to every way you can parenthesize an equation with
//      `nums.len()` integers and `nums.len() - 1` operations.
//   2. For each permutation of integers in `nums`, choose `nums.len() - 1`
//      operators from `ops`. This is done by iterating over all base
//      `ops.len()` numbers from 0 to the max `nums.len() - 1` digit number in that base.
//   3. Perform a DFS through each of the initially generated trees, substituting each
//      `num` for every leaf node and each of the selected `ops` for the current DFS
//      for each every non-leaf node.
fn solve(goal: int, nums: &[int], ops: &[&str]) {
    // Generate all trees of that have `nums.len() - 1` leaf
    // nodes. This represents every way to parenthesize an expression
    // with `nums.len()` numbers and `nums.len() - 1` operators
    let trees: Vec<Box<Tree>> = gen_trees(nums.len()).move_iter().map(|t| box t).collect();

    // For every permutation of the given numbers
    for nums_perm in nums.permutations() {
        // Calculate the max `nums.len() - 1` digit number in base `ops.len()`
        let mut range_end = 0;
        for i in range(0, nums.len() - 1) {
            range_end += pow(ops.len(), i)
        }
        range_end *= nums.len() - 1;

        for i in range(0, range_end) {
            // Use each digit of the current number in base
            // `ops.len()` to select the operations to use
            let mut digits = to_base(i, ops.len());
            while digits.len() < nums.len() - 1 {
                digits.push(0);
            }
            let ops_perm: Vec<&str> = digits.iter().map(|d| ops[*d]).collect();

            // DFS through each tree (which is a parenthesized
            // expression), replacing leaf nodes with numbers and
            // non-leaf nodes with operations. Print the result if equals the goal
            for tree in trees.iter() {
                let (eq, res) = match dfs(tree, nums_perm.as_slice(), ops_perm.as_slice()) {
                    None => continue,
                    Some((eq, res)) => (eq, res)
                };

                // TODO: Only print unique results
                if res == Ratio::from_integer(goal) {
                    println!("{} = {}", eq, res);
                }
            }
        }
    }
}

// Returns a vector of all unique full binary trees with `size` leaf nodes
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

// Converts `n` from base 10 to base `b` and returns a vector of the digits
fn to_base(mut n: uint, b: uint) -> Vec<uint> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % b);
        n = n / b;
    }
    digits.reverse();
    digits
}

// Perform a depth first search on `tree`, replacing each leaf node
// with a number from `nums`, and each non-leaf node with an operation
// from `ops`. Returns None if there is no resultant equation due to a
// divide by zero, otherwise returns an Option containing a tuple of
// the equation as a String, and the result of the equation as a
// Rational
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
