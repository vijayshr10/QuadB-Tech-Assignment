fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] > target {
            high = mid - 1;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            // Found the target, check if it's the first occurrence
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                high = mid - 1; // Continue searching for the first occurrence
            }
        }
    }

    None // Target not found
}

fn main() {
    let arr = &[1, 2, 3, 3, 4, 5];
    let target = 3;

    if let Some(index) = find_first_occurrence(arr, target) {
        println!("First occurrence of {} found at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
