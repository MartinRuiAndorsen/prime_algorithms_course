mod linear_search;
mod binary_search;
mod bubble_sort;
mod crystal_balls;

fn main() {
    let haystack = [1,2,3,4,5];
    let value = -1;

    let res = linear_search::search(&haystack, &value);
    println!("linear search res is: {res}");

    let res = binary_search::search(&haystack, 0, haystack.len(), value);
    println!("binary sort res is: {res}");

    let arr: &mut[i32;5] =  &mut[1,2,3,4,5];
    let res = bubble_sort::sort(arr);
    print!("bubble sort res is: ");
    for n in res {
        print!("{n}");
    }
    println!("");

    let breaks: &[bool; 8] = &[false, true, true, true, true, true, true, true];
    let res = crystal_balls::two_crystal_balls(breaks);
    println!("Crystal ball drops at: {res}");
}
