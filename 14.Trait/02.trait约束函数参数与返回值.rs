fn main() {
    trait Run {
        fn run(&self);
    }
    fn foo(arg: &impl Run) {
        arg.run()
    }
    // trait Bound 约束泛型参数类型，两种写法等价
    fn bar<T: Run>(arg: &T) {
        arg.run();
    }

    trait Jump {
        fn jump(&self);
    }
    // 下面三种写法等价
    fn gkd(arg: &(impl Run + Jump)) {
        arg.run();
        arg.jump();
    }
    fn mdzz<T: Run + Jump>(arg: &T) {
        arg.run();
        arg.jump();
    }

    fn ggk<T, K>(arg: &T, arg1: &K) -> ()
    where
        T: Run + Jump,
        K: Run,
    {
        arg.run();
        arg.jump();
        arg1.run();
    }
    // 使用trait约束函数的返回参数类型
    fn jntm() -> impl Run {
        struct Dog {}
        impl Run for Dog {
            fn run(&self) {}
        }
        Dog {}
    }
    // impl Run返回值只能返回一种类型
    // 这种写法会导致if两种分支返回的类型不一样，报错【可以使用enum类型处理】
    // fn jntm1() -> impl Run {
    //     let flag = true;
    //     struct Dog {}
    //     struct Cat {}
    //     impl Run for Dog {
    //         fn run(&self) {}
    //     }
    //     impl Run for Cat {
    //         fn run(&self) {}
    //     }
    //     if flag {
    //         return Dog {};
    //     } else {
    //         return Cat {};
    //     }
    // }
}
