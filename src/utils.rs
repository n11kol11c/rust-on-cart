pub fn print_vec<T: std::fmt::Debug>(vec: &[T]) {
    for item in vec {
        println!("{:?}", item);
    }
}
