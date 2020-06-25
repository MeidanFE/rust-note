struct Empty;
struct Null;

trait DroubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DroubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
