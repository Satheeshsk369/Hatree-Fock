fn main() {
    let x: Vec<i64> = (1..1000).collect();
    for i in pow6(x) {
        println!("{}", i);
    }
}

fn pow6(x: Vec<i64>) -> Vec<i64> {
    x.into_iter()
        .map(|x| i64::pow(x, 6))
        .collect()
}