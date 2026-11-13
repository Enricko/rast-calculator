//! A tiny integer calculator: `calculator "2 + 3 * (4 - 1)"`.
//! No dependencies; built for RAST's Package Center as a runtime package.

const VERSION: &str = "1.0.2";

struct Parser<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Parser {
            bytes: text.as_bytes(),
            position: 0,
        }
    }

    fn skip_space(&mut self) {
        while self
            .bytes
            .get(self.position)
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.position += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_space();
        self.bytes.get(self.position).copied()
    }

    fn take(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Some(byte)
    }

    fn number(&mut self) -> Result<i64, String> {
        self.skip_space();
        let start = self.position;
        if self.bytes.get(self.position) == Some(&b'-') {
            self.position += 1;
        }
        while self
            .bytes
            .get(self.position)
            .is_some_and(|byte| byte.is_ascii_digit())
        {
            self.position += 1;
        }
        let text = std::str::from_utf8(&self.bytes[start..self.position])
            .map_err(|_| "invalid number".to_string())?;
        text.parse::<i64>()
            .map_err(|_| format!("invalid number {text:?}"))
    }

    fn primary(&mut self) -> Result<i64, String> {
        match self.peek() {
            Some(b'(') => {
                self.take();
                let value = self.expression()?;
                if self.take() != Some(b')') {
                    return Err("missing closing parenthesis".into());
                }
                Ok(value)
            }
            Some(byte) if byte.is_ascii_digit() || byte == b'-' => self.number(),
            Some(other) => Err(format!("unexpected character {:?}", other as char)),
            None => Err("unexpected end of expression".into()),
        }
    }

    fn term(&mut self) -> Result<i64, String> {
        let mut value = self.primary()?;
        loop {
            match self.peek() {
                Some(b'*') => {
                    self.take();
                    value = value
                        .checked_mul(self.primary()?)
                        .ok_or("number too large")?;
                }
                Some(b'/') => {
                    self.take();
                    let divisor = self.primary()?;
                    if divisor == 0 {
                        return Err("division by zero".into());
                    }
                    value /= divisor;
                }
                Some(b'%') => {
                    self.take();
                    let divisor = self.primary()?;
                    if divisor == 0 {
                        return Err("division by zero".into());
                    }
                    value %= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    fn expression(&mut self) -> Result<i64, String> {
        let mut value = self.term()?;
        loop {
            match self.peek() {
                Some(b'+') => {
                    self.take();
                    value = value
                        .checked_add(self.term()?)
                        .ok_or("number too large")?;
                }
                Some(b'-') => {
                    self.take();
                    value = value
                        .checked_sub(self.term()?)
                        .ok_or("number too large")?;
                }
                _ => return Ok(value),
            }
        }
    }
}

fn evaluate(text: &str) -> Result<i64, String> {
    let mut parser = Parser::new(text);
    if parser.peek().is_none() {
        return Err("empty expression".into());
    }
    let value = parser.expression()?;
    if parser.peek().is_some() {
        return Err("trailing input after the expression".into());
    }
    Ok(value)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("Calculator {VERSION}");
        return;
    }
    if args.first().is_some_and(|arg| arg == "--help" || arg == "-h") || args.is_empty() {
        println!("Calculator {VERSION}");
        println!("usage: calculator <expression>");
        println!("  calculator \"2 + 3 * (4 - 1)\"");
        return;
    }
    let expression = args.join(" ");
    match evaluate(&expression) {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("calculator: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn precedence_and_parentheses() {
        assert_eq!(super::evaluate("2 + 3 * 4").unwrap(), 14);
        assert_eq!(super::evaluate("(2 + 3) * 4").unwrap(), 20);
        assert_eq!(super::evaluate("20 / 3").unwrap(), 6);
        assert_eq!(super::evaluate("-2 * (3 - 5)").unwrap(), 4);
        assert!(super::evaluate("1 / 0").is_err());
    }
}
