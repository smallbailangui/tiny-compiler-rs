#![allow(non_snake_case)]

#[derive(Clone, Debug)]
pub struct Edge {
    pub fromState: i32,
    pub nextState: i32,
    pub driverId: i32,
    pub DriverType: String,
}
