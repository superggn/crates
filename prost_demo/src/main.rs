mod msg;

fn main() {
    let msg = msg::SearchRequest {
        query: "query_string".to_string(),
        page_number: 100,
        results_per_page: 10,
    };
    println!("msg_1: {:?}", msg);
    let encoded = prost::Message::encode_to_vec(&msg);
    println!("encoded: {:?}", encoded);
    let decoded: msg::SearchRequest = prost::Message::decode(&*encoded).unwrap();
    println!("decoded: {:?}", decoded);
    assert_eq!(msg, decoded);
    println!("Hello, world!");
}
