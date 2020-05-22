pub fn nth(n: u32) -> u32 {

    let mut counter:u32 = 0;
    let mut p = 0;
    let t = (n * 20)+10;

    for i in 2..t {
        if prime(i) {
            if counter == n {
                p = i;
                break;
            }
            counter += 1;
        }
    }
    p
}

pub fn prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true
}
