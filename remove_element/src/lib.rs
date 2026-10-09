pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // in-place algorithm
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_remove_element() {
        // Case 1
        let mut vector_case1 = vec![3,2,2,3];
        let val_case1 = 3;
        // Check returned value
        let k = remove_element(&mut vector_case1, val_case1) as usize;
        assert_eq!(k, 2);
        // Check vector
        assert_eq!(&vector_case1[..k], vec![2,2]);

        // Case 2
        let mut vector_case2 = vec![0,1,2,2,3,0,4,2];
        let val_case2 = 2;
        // Check returned value
        let k = remove_element(&mut vector_case2, val_case2) as usize;
        assert_eq!(k, 5);
        // Check vector
        assert_eq!(vector_case2[..k], vec![0,1,4,0,3]);
    }
}
