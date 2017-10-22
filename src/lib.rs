extern crate reqwest;

fn main() {}


#[cfg(test)]
mod tests {
    #[test]
    fn simple_string() {
        let ts = serve!("hello world!");

        let response = reqwest::get(ts.url);
        let body = String::new();
        response.read_to_string(&mut body);

        assert_eq!(response.body, "hello world!");
    }

    #[test]
    fn simple_json() {
        struct data {
            key: String,
            value: String,
        }

        let ts = serve!(data {
            key: "hello",
            value: "world!",
        });

        let response = reqwest::get(ts.url);
        let body = String::new();
        response.read_to_string(&mut body);

        assert_eq!(response.body, "{\"key\":\"hello\",\"value\":\"world!\"}");
    }
}
