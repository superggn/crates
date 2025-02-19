use crate::sub1::MyStructHa1;
use crate::sub2::MyService;

impl MyService for MyStructHa1 {
    fn execute(self) {
        println!("execute for mystructha1");
    }
}
