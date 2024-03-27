pub fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jump_amount = (breaks.len() as f64).sqrt() as usize;

    let mut i = jump_amount;
    while i < breaks.len() {
        if breaks[i] {
            break;
        }
        i += jump_amount;
    }

    let mut j = 0;
    i -= jump_amount;
    while j < jump_amount && i < breaks.len() {
        if breaks[i] {
            return i as i32;
        }
        j += 1;
        i += 1;
    }
    return -1;
}