// 1. 三个互不相关的类型
struct Dog;
struct Cat;
struct Bird;

// 2. 公共行为
trait Speak {
    fn speak(&self) -> &'static str;
}

impl Speak for Dog {
    fn speak(&self) -> &'static str { "woof" }
}
impl Speak for Cat {
    fn speak(&self) -> &'static str { "meow" }
}
impl Speak for Bird {
    fn speak(&self) -> &'static str { "tweet" }
}

// 3. 把“或”关系收进一个 enum —— 这就是 Rust 的“联合类型”
enum Animal {
    Dog(Dog),
    Cat(Cat),
    Bird(Bird),
}

// 4. 统一实现，调用方不用关心内部到底是谁
impl Speak for Animal {
    fn speak(&self) -> &'static str {
        // 5. 类型缩小：match 后编译器知道确切变体
        match self {
            Animal::Dog(d) => d.speak(),
            Animal::Cat(c) => c.speak(),
            Animal::Bird(b) => b.speak(),
        }
    }
}

// 6. 泛型函数：只要实现了 Speak 就能用
fn announce<T: Speak>(creature: &T) {
    println!("{}", creature.speak());
}

fn main() {
    let pets: [Animal; 3] = [
        Animal::Dog(Dog),
        Animal::Cat(Cat),
        Animal::Bird(Bird),
    ];

    for pet in &pets {
        announce(pet);          // 统一接口，内部 match 完成缩小
    }
}

// rust没有Ts 的 泛型 | 关系，只能通过enum模拟