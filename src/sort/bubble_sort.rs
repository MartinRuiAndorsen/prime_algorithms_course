pub fn sort(numbers: &mut [i32]) -> &[i32] {
    
    let n: usize = numbers.len()-1;

    for i in 0..n {
        for j in 0..n-i {
            if numbers[j] > numbers[j+1] {
                let temp = numbers[j];
                numbers[j] = numbers[j+1];
                numbers[j+1] = temp;
            }
        }
    }

    return numbers;
}
