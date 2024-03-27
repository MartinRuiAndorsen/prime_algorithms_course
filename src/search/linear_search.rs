pub fn search(haystack: &[i32], needle: &i32) -> bool {
    for numb in haystack {
        if numb == needle {
            return true;
        }
    }
    return false;
}
