pub fn escape(value: &str) -> String {
    value.replace('\'', "''")
}
