fn main() {
    // Even Fibonacci Numbers
    // from 1 to 4.000.000
    // find the sum of the even-valued terms
    // of the fibonacci sequence
    let mut fibonacci: Vec<i32> = vec![1, 2];
    let mut even_fibonacci : Vec<i32> = vec![];
    let limit: i32 = 4000000;

    let mut i = 1;

    loop {
        let temp = fibonacci[i] + fibonacci[i - 1];
        if temp > limit {
            break;
        }
        fibonacci.push(temp);
        i += 1;
    }

    println!("Fibonacci numbers whose values do not exceed limit:");
    println!("{:?}", fibonacci);

    // find even numbers
    for i in 0..fibonacci.len() {
        if fibonacci[i] % 2 == 0 {
            even_fibonacci.push(fibonacci[i]);
        }
    }

    println!("Even Fibonacci numbers of Vector:");
    println!("{:?}", even_fibonacci);

    // sum of the even values
    let sum: i32 = even_fibonacci.iter().sum();
    println!("{}", sum);
}
