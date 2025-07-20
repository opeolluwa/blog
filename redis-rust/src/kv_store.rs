pub struct RedisKV {
    key: String,
    value: Option<String>,
}


pub struct RedisKvBuilder {
    key: String,
    value: Option<String>,
}