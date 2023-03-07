pub fn algorithm(a: i32, b: i32) -> i32 {
    if b==0 {
        return a;
    } else {
        return algorithm(b, a%b) ;
    }
}