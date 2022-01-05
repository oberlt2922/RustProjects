use std::collections::HashMap;

fn main() {
    //vars
    let nums = vec![2, 2, 3, 4, 5, 6];
    
    //calculate mean
    let mut sum = 0;
    for i in &nums {
        sum += i;
    }
    let mean = Some(sum as f32 / nums.len() as f32);
    println!("mean: {:?}", mean);
    
    //calculate median
    let median = if nums.len() % 2 == 0 {Some(nums.len() as i32 / 2)} else {Some(nums.len() as i32 / 2 + 1)};
    println!("median: {:?}", median);
    
    //calculate mode
    let mut mode: Option<i32> = None;
    let mut max_count: Option<i32> = None;
    let mut map = HashMap::new();
    for i in &nums {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    for(k, count) in map {
        match max_count {
            Some(i) => {
                if count > i {
                    mode = Some(*k);
                    max_count = Some(count);
                }
            },
            None => {
                mode = Some(*k);
                max_count = Some(count);
            }
        };
    }
    println!("{:?}", mode);
}
