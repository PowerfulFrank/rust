pub fn raindrops(n: u32) -> String {

    let mut s = "".to_string();
    let values = [(3, "Pling"),(5, "Plang"),(7, "Plong")];

    for (i, sound) in values.iter() {
        if n % i == 0 {
            s.push_str(sound);
        }
    }

    if s.is_empty(){
        s = n.to_string();
    }

    s
}
