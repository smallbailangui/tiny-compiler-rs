#![allow(non_snake_case)]
#![allow(dead_code)]

pub fn lab2test() -> i64 {
    testTask3();
    0
}

pub fn testTask1() {
    crate::lab2::task1::test1();
    crate::lab2::task1::get_All_NTS_FIRST();
    crate::lab2::task1::show_All_NTS_FIRST();
    crate::lab2::task1::get_All_Production_FIRST();
    crate::lab2::task1::show_All_Production_FIRST();
    crate::lab2::task1::get_All_NTS_FOLLOW();
    crate::lab2::task1::show_All_NTS_FOLLOW();
}

pub fn testTask2() {
    crate::lab2::task2::test2();
    crate::lab2::task2::test2_1();
    crate::lab2::task2::test2_4();
    crate::lab2::task2::test2_5();
    crate::lab2::task2::test2_6();
}

pub fn testTask3() {
    crate::lab2::task3::reset_lab2_state();
    println!("开始构建 TINY 文法...");
    crate::lab2::task3::create_TINY_Grammar();
    println!("开始构建 TINY LR(0) DFA...");
    crate::lab2::task3::create_TINY_LR0_DFA();
    println!("开始进行 TINY 语法分析测试...");
    crate::lab2::task3::test_TINY3();
}
