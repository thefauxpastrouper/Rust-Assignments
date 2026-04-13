// Option Handling: Chain multiple Option operations safely

fn main() {
    let value = Some(4);

    let result = value.and_then(|x| if x>0 {Some(x*2)} else {None})
                        .and_then(|x| Some(x+1));

    println!("{:?}", result);
}
