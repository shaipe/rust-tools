pub struct AccessToken {
    pub db_id: u32,
    pub proprietor: u32,
    pub proprietor_id: u64,
    pub fk_flag: u32,
    pub fk_fkid: u64,
    pub user_name: String,
    pub domain: String,
    pub token_type: u32,
    pub expired: u64,
}

impl AccessToken {
    pub fn new(_fk_id: u64, _fk_flag: u32, _db_id: u32, _user_name: &str) -> Self {
        AppOrder {
            db_id: _db_id,
            fk_id: _fk_id,
            fk_flag: _fk_flag,
            user_name: _user_name.to_owned(),
            domain: String::from(""),
            tokenType: 0,
        }
    }
}
