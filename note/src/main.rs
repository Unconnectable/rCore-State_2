fn main() {
    /* let source = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let closure = |x| x * x;
    let output: Vec<_> = source
        .iter()
        .filter(|x: &&i32| **x % 2 == 0)
        .map(closure)
        .collect();
    println!("{:?}", output); // [4, 16, 36, 64] */

    //println!("\x1b[32m output is {:?} \x1b[0m", out);     

    /* let mut source = vec![1, 2, 3, 4];
    for i in source.iter_mut() {
        *i = *i * 3;
        println!("{}", i);
    }
    println!("{:?}", source); */

    let source: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let output = source.iter().sum::<i32>();
    println!("{:?}", output);
}
