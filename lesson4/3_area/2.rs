fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut total = 0;
    for num in nums {
        match num.checked_add(total) {
            Some(new_total) => total = new_total,
            None => return None,
        }
    }
    Some(total)
}