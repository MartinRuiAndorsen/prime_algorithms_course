// Binary search, we must have an ordered array
// low -> high is the range of possible values
pub fn search(ordered_array: &[i32], mut low: usize, mut high: usize, numb: i32) -> bool {
    while low < high {
        let mid_index: usize = low+(high-low)/2;
        let mid_value = ordered_array[mid_index];
        if numb == mid_value {
            return true;
        } else if mid_index == 0 {
            return false;
        } else if numb > mid_value {
            low = mid_index+1;
        } else {
            high = mid_index;
        }
    }
    return false;
}
