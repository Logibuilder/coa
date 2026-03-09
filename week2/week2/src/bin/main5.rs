/// Returns a Vec of random values
fn get_rand_values(seed: &mut u32, count: usize) -> Vec<u32> {
    let mut v = Vec::new();
    for _ in 0..count {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 17;
        *seed ^= *seed << 5;
        v.push(*seed);
    }
    v
}

fn main() {

    let mut best : Option<Vec<u32>> = None;
    let mut best_sum = 0;
    let mut seed = 123;
    for _ in 0..10 {
        let rand_values = get_rand_values(&mut seed, 10);

        
        let mut sum: u32 = 0;

        for val in &rand_values {
            // prevent panic in debug mode when sum wraps
            sum = sum.wrapping_add(*val);
        }
        if sum > best_sum {
            best = Some(rand_values);
            best_sum = sum;
        }
    }
    println!("{best:#?}");
    
}
