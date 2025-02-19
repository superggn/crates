use sub1::MyStructHa1;

pub use crate::*;
mod mysub2;

// pub fn myfunc() {
//     println!("mystruct1: {:?}", MyStructHa1);
// }
// impl std::Display for MyStructHa1;
pub trait MyService {
    /// 处理 Command，返回 Response
    fn execute(self);
}
