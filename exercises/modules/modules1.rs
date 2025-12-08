mod sausage_factory {
    // 保持私有（外部不可见）
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 用 pub 修饰，让模块外可以调用
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
