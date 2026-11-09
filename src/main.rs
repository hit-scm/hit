fn main() {
    if let Err(err) = hit::run() {
        err.print_and_exit();
    }
}
