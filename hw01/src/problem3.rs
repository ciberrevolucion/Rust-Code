/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    // create vector with range 2 to n
    for i in 2..n+1 {
        vec.push(i as u32);
    }
    // Mark multiples on a vector of booleans
    let mut is_multiple: Vec<bool> = vec![false; vec.len()];
    let mut x: u32 = 2;
    let mut ind: usize;
    for i in 0..vec.len() {
        if is_multiple[i] == false {
            println!("Found prime {:?}", vec[i]);
            while vec[i]*x <= n {

                ind = ((i as u32 +2)*x-2 ) as usize; // vec[i] = i + 2
                is_multiple[ind] = true;
                x += 1;
            }
            x = 2; //reset the x for next prime
        }
    }
    // Compose result vector
    let mut result: Vec<u32> = Vec::new();
    for i in 0..vec.len() {
        if !is_multiple[i] {
            result.push(vec[i]);
        }
    }
    result
}
