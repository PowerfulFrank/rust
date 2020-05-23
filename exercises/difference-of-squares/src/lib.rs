pub fn square_of_sum(n: u32) -> u32 {
    let sum:u32 = (n*(n+1))/2;
    return sum.pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    return (n * (n + 1) * (2 * n + 1)) / 6;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
