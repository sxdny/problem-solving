fn main() {
    let multiples: [i32; 2] = [3, 5];
    let mut timesMultiples: Vec<_> = vec![0];
    let limit = 1000;
    for i in 1..limit {
        for j in 0..multiples.len() {
            let temp = multiples[j] * i;
            if temp < limit {
                // some multiples can exists twice (5 * 3, 3 * 5)
                // we need to avoid duplicates, also the result
                // would be incorrect -> 266.333
                if !timesMultiples.contains(&temp) {
                    timesMultiples.push(temp);
                }
            }  
        }
    }
    println!("{:?}", timesMultiples);
    let sum: i32 = timesMultiples.iter().sum();
    println!("{}", sum);
}
