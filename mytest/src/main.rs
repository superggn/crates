trait MyTrait {
    fn greet(&self) -> String;
}

struct MyStruct {
    name: String,
}

impl MyTrait for MyStruct {
    fn greet(&self) -> String {
        format!("hello, {}", self.name)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_trait() {
//         let obj = MyStruct {
//             name: "hahaha".into(),
//         };
//         assert_eq!(obj.greet(), "hello, hahaha".to_string());
//     }
// }

#[cfg(test)]
mod mytest {
    use super::*;

    #[test]
    fn test_trait_3() {
        let obj = MyStruct {
            name: "hahaha".into(),
        };
        assert_eq!(obj.greet(), "hello, hahaha".to_string());
    }
}

#[test]
fn test_haha() {
    let obj = MyStruct {
        name: "hahaha".into(),
    };
    assert_eq!(obj.greet(), "hello, hahaha".to_string());
}

fn main() {}
