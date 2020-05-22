pub fn build_proverb(list: &[&str]) -> String {

    let mut together = vec![];
    let n = list.len();

    if n == 0 {
        String::new()
    } else {
        for i in 0..n-1 {
            together.push(format!("For want of a {} the {} was lost.",list[i], list[i+1]));
        }

        together.push(format!("And all for the want of a {}.",list[0]));

        together.join("\n")
    }

}
