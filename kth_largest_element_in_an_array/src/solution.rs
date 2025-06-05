use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums.clone());

    for _ in 0..k - 1 {
        heap.pop();
    }
    *heap.peek().unwrap()
}
