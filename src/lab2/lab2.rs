#![allow(dead_code)]

mod types;
mod grammar;
mod first;
mod follow;
mod ll1;
mod lr0;
mod slr;
mod display;
mod report;
mod tests;
mod runner;

fn main() {
    runner::run_lab2();
}
