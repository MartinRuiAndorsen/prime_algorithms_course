pub fn sort(numbers: &mut [i32]) -> &[i32] {
    
    let mut end: usize = numbers.len() -1;
    while end > 1 {
        for n in 0..end {
            if numbers[n] > numbers[n+1] {
                let temp = numbers[n];
                numbers[n] = numbers[n+1];
                numbers[n+1] = temp;
            }
        }
        end -= 1;
    }
    return numbers;
}
