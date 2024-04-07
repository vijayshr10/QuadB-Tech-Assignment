fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = i32::MIN; // Initialize with minimum possible value
    let mut current_sum = 0;
  
    for num in arr {
      current_sum = std::cmp::max(*num, current_sum + *num); // Update current_sum
      max_sum = std::cmp::max(max_sum, current_sum); // Update max_sum if current_sum is larger
    }
  
    max_sum
  }
  `1
  fn main() {
    let arr = &[1, -2, 3, 4, -1, 2, 1, 5, -3];
    let max_sum = max_subarray_sum(arr);
  
    println!("Maximum subarray sum: {}", max_sum);
  }
  