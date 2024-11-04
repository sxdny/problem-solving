fn main() {
    // Find the largest palindrome made from the product of two 3-digit numbers.
    // Project Euler - Problem 4

    // Tengo que encontrar el n palindromo más grande, hecho con
    // la multiplicación de 2 números de 3 dígitos. (100 - 999)

    let mut nPalindrome: i64 = 1001;
    let mut palindromresVec : Vec<i64> = vec![];

    // for loop multiplicando, verificando y guardando
    println!("{}", isPalindrome(nPalindrome));

    for i in 100..999 {
        let temp: i64 = i;
        for j in 100..999 {
            if isPalindrome(temp * j) {
                palindromresVec.push(temp * j);
            }
        }
    }

    palindromresVec.sort();
    let l = palindromresVec[palindromresVec.len() - 1];
    println!("Largest Palindrome: {}", l);



}

fn isPalindrome(n: i64) -> bool {
    // pasamos n a string:
    let n_asString = n.to_string();
    let string_reversed = n_asString.chars().rev().collect::<String>();

    // comparamos las 2 string
    if n_asString == string_reversed {
        return true;
    } else {
        return false;
    }

}
