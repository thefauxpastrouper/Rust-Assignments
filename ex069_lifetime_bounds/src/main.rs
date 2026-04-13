use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

struct RefHolder<'a, T> {
    value: &'a T
}

impl <'a , T: 'a + Debug> Debug for RefHolder<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("RefHolder")
        .field("value", &self.value)
        .finish()
    }    
}

fn main() {
    let value = 42;
    let reference = RefHolder { value: &value };
    println!("{:?}", reference);
}
