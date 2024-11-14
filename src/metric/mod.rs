pub struct Metric {
    name: String,
    values: Vec<KeyValue>,
}

pub struct KeyValue {
    pub key: String,
    pub value: String,
}