use uuid::Uuid;

fn main() -> Result<(), uuid::Error> {
    let now = std::time::Instant::now();

    let mut uids = Vec::new();
    for _ in 0..10 {
        let my_uuid = uuid_v4();
        println!("{}", my_uuid.len());
        uids.push(my_uuid);
    }

    let end = now.elapsed();
    println!(
        "use time: {}.{:03}.{}s",
        end.as_secs(),
        end.subsec_millis(),
        end.subsec_nanos()
    );
    // let dt = d.as_secs() * 1_000_000_000 + u64::from(d.subsec_nanos());

    println!("{:?}", uids);
    Ok(())
}

fn uuid_v5() -> String {
    let ns = Uuid::NAMESPACE_X500;
    // 如果name一样将取出的值是一样的
    let uuid5= Uuid::new_v5(&ns, "name".as_bytes());

    uuid5.to_string()
}

/// uuid v4
fn uuid_v4() -> String {
    format!("{}{}{}", "dre", "class", Uuid::new_v4().to_simple().to_string())
    // Uuid::new_v4().to_simple().to_string()
}

// uuid v3
fn uuid_v3() -> String{
    let ns = Uuid::NAMESPACE_X500;
    // 如果name一样将取出的值是一样的
    let uuid3= Uuid::new_v3(&ns, "name".as_bytes());

    uuid3.to_string()

}

/// uuid v1 vreion 
fn uuid_v1() -> String {
    use uuid::v1::{Context, Timestamp};
    let time: u64 = 1_496_854_535;
    let time_fraction: u32 = 812_946_000;
    let node = [1, 2, 3, 4, 5, 6];
    let context = Context::new(0);

    let uuid2 = Uuid::new_v1(Timestamp::from_unix(&context, time, time_fraction), &node).unwrap();

    uuid2.to_hyphenated().to_string()
}
