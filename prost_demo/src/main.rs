use abi::{
    MyStudent,
    my_student::{ExtraData, Grade},
};
use prost::Message;
use prost_types::Timestamp;

pub mod abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}
fn main() {
    // encode + decode
    let stu = MyStudent {
        name: "andy".to_string(),
        // 注意， 这里的 size 虽然定义是 enum， 但并没有严格限制取值为 0/1/2
        // 即使这里用 10 也是合法的， 只是没有对应的 string name 而已
        // grade: 10,
        grade: Grade::Freshman.into(),
        // extra_data: Some(ExtraData::Age(19)),
        extra_data: Some(ExtraData::Address("断罪小学".to_string())),
    };
    println!("stu: {:?}", stu);
    let mut buf = Vec::new();
    stu.encode(&mut buf).unwrap();
    println!("encoded stu: {:?}", buf);
    let transferred_stu = MyStudent::decode(&buf[..]).unwrap();
    println!("transferred_stu: {:?}", transferred_stu);
    assert_eq!(stu, transferred_stu);
    println!("assertion passed!");
    // serde compatibility
    let ser = serde_json::to_string(&stu).unwrap();
    println!("ser json: {:?}", ser);
    let des: MyStudent = serde_json::from_str(&ser).unwrap();
    println!("des json: {:?}", des);
    assert_eq!(stu, des);
    println!("assertion passed!");
    // prost-types => WKT (Well-Known-Types)
    let t1 = Timestamp::date_time(2025, 3, 4, 0, 0, 0).unwrap();
    println!("t1: {:?}", t1);
    let mut buf_t = Vec::new();
    t1.encode(&mut buf_t).unwrap();
    println!("encoded timestamp: {:?}", buf_t);
    let t2 = Timestamp::decode(&buf_t[..]).unwrap();
    println!("decoded timestamp: {:?}", t2);
    assert_eq!(t1, t2);
    println!("assertion passed!");
}
