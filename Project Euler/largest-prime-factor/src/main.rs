fn main() {
    // Find the largest prime factor
    // of th number: 600-851-475-143

    let mut n: i64 = 600851475143;
    let mut factors : Vec<i32> = vec![];

    // Encontramos la raíz cuadrada de n
    let sqrtn = (n as f64).sqrt().floor();
    let int_sqrtn = sqrtn as i64;

    println!("Squareroot of n: {:?}", sqrtn);

    // Ahora dividimos hasta la sqrt
    for i in 1..int_sqrtn {
        if n % i == 0 { // 5
            // Aqui ya entran los factores
            // Ahora comprobamos que sean primos...
            if is_prime(i) {
                factors.push(i.try_into().unwrap());
            }
        }
    }

    println!("Prime Factors: {:?}", factors);

    // Ahora encontramos el valor más alto de la array.
    // comparando numero a numero
    let l = factors[factors.len() - 1];

    println!("Largest number: {}", l);
}

fn is_prime(n: i64) -> bool {
    // Funcion para comprobar que un número es primo
    let mut dividers : Vec<i64> = vec![n];
    for i in 1..n {
        if n % i == 0 {
            dividers.push(i);
        }
    }

    // Comprobamos el len de dividers
    if dividers.len() <= 2 {
        return true;
    } else {
        return false;
    }

    
}
