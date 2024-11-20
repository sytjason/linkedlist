mod first;
use first::{List, print_list};
fn main() {
    let mylist: List = List::Elem(1, Box::new(List::Empty));
    print_list(mylist);
}
