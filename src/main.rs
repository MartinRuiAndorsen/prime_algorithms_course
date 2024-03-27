mod search;
mod sort;

fn main() {
    let haystack = [1,2,3,4,5];
    let value = -1;

    let res = search::linear_search::search(&haystack, &value);
    println!("linear search res is: {res}");

    let res = search::binary_search::search(&haystack, 0, haystack.len(), value);
    println!("binary sort res is: {res}");

    let breaks: &[bool; 8] = &[false, true, true, true, true, true, true, true];
    let res = search::crystal_balls::two_crystal_balls(breaks);
    println!("Crystal ball drops at: {res}");

    let arr: &mut[i32;6] =  &mut[0, 2,1,4,7,0];
    let res = sort::bubble_sort::sort(arr);
    print!("bubble sort res is: ");
    for n in res {
        print!("{n}");
    }
    println!("");
}
