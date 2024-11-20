#[derive(Debug)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

pub fn print_list(_list: List) {
    println!("{:?}", _list);
}
