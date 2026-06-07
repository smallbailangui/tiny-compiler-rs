#[path = "../lab1/mod.rs"]
mod lab1;
#[path = "../lab3/mod.rs"]
mod lab3;
#[path = "../lab4/mod.rs"]
mod lab4;

fn main() {
    if let Err(err) = lab4::run_lab4_test() {
        eprintln!("实验 4 运行失败: {}", err);
        std::process::exit(1);
    }
}