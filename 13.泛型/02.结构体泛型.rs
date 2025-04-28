fn main() {
    struct Point<T, K> {
        x: T,
        y: K,
    }
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: "1", y: "2" };
}
