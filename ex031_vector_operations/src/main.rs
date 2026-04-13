// Vector Operations: Implement common algorithms on Vec<T>
fn main() {
    let mut v = vec![1,2,3,4];

    v.push(4);
    v.insert(2,5);
    v.pop();
    v.remove(0);

    println!("{:?}", v);

    for item in &v {println!("{}", item)}
    for item in &mut v {*item *= 2;}
    for item in v { println!("{}", item);}

    let c = vec![3,3,2,1];
    
    let doubled:Vec<i32> = c.clone().iter().map(|x| x*2).collect();
    let evens:Vec<i32> = c.clone().into_iter().filter(|x| x % 2 == 0).collect();
    let sum:i32 = c.clone().iter().sum::<i32>();

    println!("original: {:?} doubled: {:?} evens: {:?} sum: {:?}", c, doubled, evens, sum);

    let mut t = vec![3,3,2,1];
    let mut p = t.clone();
    let mut q = t.clone();

    t.reverse();
    println!("Reversed: {:?}", t);

    p.sort();
    println!("Sorted: {:?}", p);

    q.dedup();
    println!("Deduped: {:?}", q);
}
