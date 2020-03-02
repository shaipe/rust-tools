use chrono::prelude::*;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};

extern crate chrono;
pub struct AccessToken {
    pub db_id: u16,
    pub proprietor: u32,
    pub proprietor_id: u64,
    pub fk_flag: u32,
    pub fk_id: u64,
    pub user_name: String,
    pub domain: String,
    pub token_type: u32,
    pub expired: u64,
}

impl AccessToken {
    pub fn new(_fk_id: u64, _fk_flag: u32, _db_id: u16, _user_name: &str) -> Self {
        AccessToken {
            db_id: _db_id,
            proprietor: 0,
            proprietor_id: 0,
            fk_id: _fk_id,
            fk_flag: _fk_flag,
            user_name: _user_name.to_owned(),
            domain: String::from(""),
            token_type: 0,
            expired: 0,
        }
    }
    pub fn to_token_string(&self) -> String {
        // let dt = Local::now() + Duration::minutes(1);
        // let ticket = dt.timestamp() * 10000 + 621355968000000000;

        let ticket2 = self.get_ticks();

        //println!("{:?}，{:?}", ticket, ticket2);
        let ccc = format!(
            "{dbid}_0_0_{fkflag}_{fkid}_{}__0_{ticket}",
            dbid = self.db_id,
            fkflag = self.fk_flag,
            fkid = self.fk_id,
            ticket = ticket2
        );
        lane_crypto::aes_encrypt(ccc)
    }
    fn get_ticks(&self) -> i64 {
        let c = Local::now().naive_local() + Duration::minutes(10);
        let s = NaiveDate::from_ymd(0001, 1, 1).and_hms(00, 00, 00);

        // println!("{:?}", Utc.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 000_000));
        // println!("{:?}", Local::now());

        let x: Duration = c - s;

        let x1 = x.num_microseconds(); //.unwrap() * 1000;
        x1.unwrap() * 10
    }
}
