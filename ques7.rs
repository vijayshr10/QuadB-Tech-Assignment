fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    // Handle invalid input (k out of bounds or empty array)
    if k == 0 || k > arr.len() {
      return None;
    }
  
    // Perform selection sort to get the kth smallest element
    for i in 0..arr.len() - 1 {
      let mut min_index = i;
      for j in i + 1..arr.len() {
        if arr[j] < arr[min_index] {
          min_index = j;
        }
      }
      arr.swap(i, min_index);
    }
  
    // Return the kth element (k - 1 because indexing starts from 0)
    Some(arr[k - 1])
  }
  
  fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
  
    if let Some(element) = kth_smallest(&mut arr, k) {
      println!("{}th smallest element: {}", k, element);
    } else {
      println!("Invalid k value");
    }
  }
  