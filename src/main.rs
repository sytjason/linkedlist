mod first;
use first::{List};
fn main() {
    let mut list = List::new();
    list.push(2);
    list.push(32);
    list.push(33);
    list.push(39);
    list.pop();
    list.print_list();
}
