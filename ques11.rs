fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged_vec = Vec::with_capacity(arr1.len() + arr2.len());
  
    let mut i = 0;
    let mut j = 0;
  
    while i < arr1.len() && j < arr2.len() {
      if arr1[i] < arr2[j] {
        merged_vec.push(arr1[i]);
        i += 1;
      } else {
        merged_vec.push(arr2[j]);
        j += 1;
      }
    }
  
    // Add remaining elements from either array if one finishes first
    merged_vec.extend_from_slice(&arr1[i..]);
    merged_vec.extend_from_slice(&arr2[j..]);
  
    merged_vec
  }
  
  fn main() {
    let arr1 = &[1, 3, 5];
    let arr2 = &[2, 4, 6];
    let merged_array = merge_sorted_arrays(arr1, arr2);
  
    println!("Merged array: {:?}", merged_array);
  }
  