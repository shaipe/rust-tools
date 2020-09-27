use serde_json::Value;

fn main() {

    let v: Value = serde_json::from_str(r#"{
        "test":2,
        "t": "we"
    }"#).unwrap();

    for k in v.as_object().unwrap().keys() {
        println!("{}, {:?}", k, v[k]);
    }

    println!("{}", v.to_string());

    // println!("{:?}", v.as_object().unwrap().keys().unwrap());
}
