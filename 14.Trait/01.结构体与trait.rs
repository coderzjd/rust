fn main() {
    struct Dog {
        name: String,
        height: f32,
    }
    struct Car {
        car_name: String,
        width: f32,
    }

    // 定义trait
    trait GetName {
        // get_name方法只有签名没有默认实现
        fn get_name(&self) -> ();
        // get_info有默认实现
        fn get_info(&self) -> () {
            println!("get_info 的内置实现");
            // 可以在trait定义内部调用其他实现的方法
            self.get_name()
        }
    }
    // 为结构体实现trait
    impl GetName for Dog {
        fn get_name(&self) {
            println!("get_name is {}", &self.name)
        }
    }
    impl GetName for Car {
        fn get_name(&self) {
            println!("get_name is {}", &self.car_name)
        }
        // 覆盖默认实现
        fn get_info(&self) -> () {
            println!("覆盖默认实现 get_info");
            // 可以在实现trait时调用其他实现的方法
            self.get_name();
        }
    }
    let d1 = Dog {
        name: String::from("旺财"),
        height: 0.5,
    };
    let c1 = Car {
        car_name: String::from("凯迪拉克"),
        width: 1.98,
    };
    d1.get_name();
    c1.get_name();
    d1.get_info();
    c1.get_info();

    trait Run {
        // 含有self参数的函数可以被结构体实例调用
        fn run(&self) {
            println!("run");
        }
        // 不含有self的参数的函数，只能通过结构体调用（关联方法）
        fn out() {
            println!("998")
        }
    }
    struct People {
        name: String,
    }
    // 当trait方法有默认实现时，可以不实现方法
    // 当trait方法没有默认实现时，必须实现改方法
    impl Run for People {}
    let p1 = People {
        name: String::from("张三"),
    };
    p1.run();
    // 为结构体实现一个trait时,结构体调用方法，方法必须有一个“self”参数，out()无法调用
    // p1.out()
    People::out();
}
