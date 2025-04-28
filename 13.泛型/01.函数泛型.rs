fn main() {
    fn foo<T>(arg: &[T]) -> &T {
        &arg[100]
    }
    let arr1 = [1, 2, 3];
    let info = foo(&arr1);
    println!("info is {info}");

    let arr2 = ['1', '2', '3'];
    let info = foo(&arr2);
    println!("info is {info}");
}
