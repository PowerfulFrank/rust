pub fn verse(n: u32) -> String {
    let startvalue = 99;
    if n > 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1 )
    } else if n == 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1 )
    } else if n == 1 {
        format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n )
    } else {
        format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n", startvalue )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song:String = "".to_string();

    for num_beers in (end..=start).rev() {
        song.push_str(&verse(num_beers));
        if num_beers > end{
            song.push_str("\n");
        }
    }
    return song;
}
