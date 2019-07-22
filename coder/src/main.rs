// #[macro_use]
extern crate mysql;

use mysql as my;

mod config;

fn main() {

    let c = config::Config::new("config.json");
    println!("{:?}", c);

    let dbc = c.database;

    let conn_str = format!("mysql://{}:{}@{}:{}/{}", dbc.user, dbc.password, dbc.server, dbc.port, dbc.db_name);

    let pool = my::Pool::new(conn_str).unwrap();

    // pool.prep_exec("SELECT * from ehr_category", ())
    // .map(|result|{
    //     result.map(|x| x.unwrap())
    //     .map(|row| {
    //         let (categoryname) = my::from_row(row);
    //         println!("{:?}", categoryname);
    //         categoryname
    //     }).collect()
    // }).unwrap();
    // println!("{:?}", xx);

    // Let's select payments from database
    let selected_payments: Vec<String> =
    pool.prep_exec("SELECT * from ehr_category", ())
    .map(|result| { 
        // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        println!("{:?}", result);
        result.map(|x| x.unwrap()).map(|row| {
            let (categoryname) = my::from_row(row);
             categoryname
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    println!("{:?}", selected_payments);

    // // Let's create payment table.
    // // It is temporary so we do not need `tmp` database to exist.
    // // Unwap just to make sure no error happened.
    // pool.prep_exec(r"CREATE TEMPORARY TABLE tmp.payment (
    //                      customer_id int not null,
    //                      amount int not null,
    //                      account_name text
    //                  )", ()).unwrap();


    println!("Hello, world!");
}
