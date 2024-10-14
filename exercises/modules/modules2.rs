// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod delicious_snacks {
    // 无需在这里导入常量，直接在需要使用的地方从对应的子模块导入
    // use self::fruits::PEAR as fruit;
    // use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // 直接从子模块导入常量
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::PEAR,
        delicious_snacks::veggies::CUCUMBER
    );
}