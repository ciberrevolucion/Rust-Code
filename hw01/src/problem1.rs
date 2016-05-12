pub fn sum(slice: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for i in slice {
        result += *i;
    }
    result
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for i in vs {
        if v.contains(i) {
            continue;
        } else {
            v.push(*i);
        }
    }
    v
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for i in vs {
        if pred(*i) {
            v.push(*i);
        }
    }
    v
}   
