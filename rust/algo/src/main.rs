extern crate rand;

//use std::os::macos::raw::stat;

mod arrays;
use arrays::balanced_parenthes::Solution;

fn main() {
    let result = Solution::build_all(5);

    for el in result {
        println!("{:}", el)
    }

    //let mut nums = vec![2, 3, 7, 6, 8, -1, -10, 15]; //1
    //let mut nums = vec![2, 3, -7, 6, 8, 1, -10, 15]; //4
    //let mut nums = vec![1, 1, 0, -1, -2]; //2
    //smallest_missing_positive_number(&mut nums);
}
