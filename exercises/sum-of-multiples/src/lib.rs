pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let mut factorsv: Vec<u32> = factors.to_vec();
    let temp = factorsv.clone();

    let mut vec_of_vecs = vec![];

    factorsv.sort();

    for fv in temp.iter() {
        if *fv != 0 {
            factorsv.retain(|number| number % fv != 0 || number == fv);
            factorsv.sort();
        } else {
            factorsv.push(0);
        }
    }

    for f in factorsv.iter() {
        if *f != 0 {
            let mut f_vec = Vec::new();
            for fj in 1..limit {
                if fj*f < limit {
                    f_vec.push(fj*f);
                } else {
                    break;
                }
            }
            vec_of_vecs.extend(f_vec);
        } else {
            vec_of_vecs.push(0);
        }
    }
    vec_of_vecs.sort();
    println!("{:?}",vec_of_vecs);
    vec_of_vecs.dedup();
    println!("{:?}",vec_of_vecs);

    vec_of_vecs.iter().sum()
}
