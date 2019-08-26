use startLine;
use std::error::Error;
use std::str;

#[derive(Debug)]
pub struct Message<'a> {
    startLine: &'a str,
    headers: &'a str,
    body: &'a str,
}

impl<'a> Message<'a> {
    pub fn from_request(request: &'a [u8]) {
        let message = str::from_utf8(request).unwrap();

        for (index, message) in message.split("\r\n").enumerate() {
            match index {
                0 => startLine::from_string(message),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
