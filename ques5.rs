fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
  
    // Handle empty or single-element arrays (median is undefined)
    if len == 0 {
      return f64::NAN; // Not a number
    }
  
    let mid_index = len / 2;
  
    // Calculate median based on array length (even or odd)
    if len % 2 == 0 {
      // Even number of elements, median is the average of middle two elements
      let median = (arr[mid_index - 1] as f64 + arr[mid_index] as f64) / 2.0;
      median
    } else {
      // Odd number of elements, median is the middle element
      arr[mid_index] as f64
    }
  }
  
  fn main() {
    let arr = &[1, 3, 5, 7, 9];
    let median = find_median(arr);
  
    if median.is_nan() {
      println!("Empty array, median undefined");
    } else {
      println!("Median of the array: {}", median);
    }
  }
  