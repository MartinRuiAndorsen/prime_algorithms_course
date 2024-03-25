mod linear_search;
mod binary_search;
mod crystal_balls;

fn main() {
    let haystack = [1,2,3,4,5];
    let value = -1;

    let res = linear_search::search(&haystack, &value);
    println!("Res is: {res}");

    let res = binary_search::search(&haystack, 0, haystack.len(), value);
    println!("res is: {res}");

    let breaks: &[bool; 8] = &[false, true, true, true, true, true, true, true];
    let res = crystal_balls::two_crystal_balls(breaks);
    println!("Crystal ball drops at: {res}");
}
