
#[allow(dead_code)]
pub fn smallest_missing_positive_number(nums: &mut Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let num = nums[i] % (nums.len() as i32);
        if num > 0 && num < nums.len() as i32 {
            if nums[num as usize - 1] < 0 {
                nums[num as usize - 1] = 0;
            }
            nums[num as usize - 1] = nums[num as usize - 1] + (nums.len() as i32);
        }
    }
    for i in 0..nums.len() {
        let num = nums[i];
        if num > 0 && num < nums.len() as i32 {
            return i as i32 + 1;
        }
    }
    return -1;
}

pub fn deleted_number(nums: &mut Vec<i32>) -> i32 {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_missing_positive_number() {
        let mut nums = vec![2, 3, 7, 6, 8, -1, -10, 15];
        assert_eq!(smallest_missing_positive_number(&mut nums), 1);

        let mut nums = vec![2, 3, -7, 6, 8, 1, -10, 15];
        assert_eq!(smallest_missing_positive_number(&mut nums), 4);

        let mut nums = vec![1, 1, 0, -1, -2];
        assert_eq!(smallest_missing_positive_number(&mut nums), 2);
    }

}
