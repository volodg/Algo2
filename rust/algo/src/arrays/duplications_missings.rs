
//use smallest_missing_positive_number;
//https://www.geeksforgeeks.org/find-the-smallest-positive-number-missing-from-an-unsorted-array/
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

#[allow(dead_code)]
pub fn deleted_number(nums: &Vec<i32>) -> i32 {
    let expected = ((1 + nums.len() + 1) * (nums.len() + 1) / 2) as i32;
    expected - nums.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deleted_number() {
        assert_eq!(deleted_number(&vec![1, 2]), 3);
        assert_eq!(deleted_number(&vec![1, 3]), 2);
        assert_eq!(deleted_number(&vec![2, 3]), 1);
    }

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
