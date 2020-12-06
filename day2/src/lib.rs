#[derive(Debug)]
pub struct PasswordPolicy {
    min: u8,
    max: u8,
    c: char,
}

#[derive(Debug)]
pub struct Password<'a> {
    policy: PasswordPolicy,
    pass: &'a str,
}

impl<'a> Password<'a> {
    pub fn valid_a(&self) -> bool {
        let mut c_count = 0;

        for c in self.pass.chars() {
            if c == self.policy.c {
                c_count += 1;
            }
        }

        return c_count >= self.policy.min && c_count <= self.policy.max;
    }

    pub fn valid_b(&self) -> bool {
        let pol = &self.policy;
        let bytes = self.pass.as_bytes();

        if pol.max as usize > self.pass.len() {
            false
        } else {
            let first_c = bytes[(pol.min - 1) as usize] as char;
            let last_c = bytes[(pol.max - 1) as usize] as char;

            return pol.c != first_c && pol.c == last_c || pol.c == first_c && pol.c != last_c;
        }
    }
}

pub struct ParseContext<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> ParseContext<'a> {
    pub fn new(text: &'a str) -> ParseContext {
        ParseContext {
            bytes: text.as_bytes(),
            index: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.index < self.bytes.len() {
            Some(self.bytes[self.index])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<u8> {
        if self.index + 1 < self.bytes.len() {
            let curr_byte = self.bytes[self.index];
            self.index += 1;
            Some(curr_byte)
        } else {
            None
        }
    }

    fn skip_or_fail(&mut self, c: u8) -> Option<()> {
        if self.peek()? != c {
            None
        } else {
            let _ = self.next()?;
            Some(())
        }
    }

    fn trim_spaces(&mut self) -> Option<()> {
        while self.peek()? == b' ' {
            let _ = self.next()?;
        }
        Some(())
    }

    fn num(&mut self) -> Option<u64> {
        let mut n: u64 = 0;
        let mut mul: u64 = 1;
        loop {
            match self.peek()? {
                num @ b'0'..=b'9' => {
                    n *= mul;
                    n += (num - 48) as u64;
                    mul *= 10;

                    match self.next() {
                        Some(_) => continue,
                        None => break,
                    }
                }
                _ => break,
            }
        }

        if n == 0 {
            None
        } else {
            Some(n)
        }
    }

    fn alphas(&mut self) -> Option<&'a [u8]> {
        let start_index = self.index;

        loop {
            match self.peek()? {
                b'a'..=b'z' => match self.next() {
                    Some(_) => continue,
                    None => break,
                },
                _ => break,
            }
        }

        if start_index == self.index {
            None
        } else {
            Some(&self.bytes[start_index..=self.index])
        }
    }

    pub fn parse_password(&mut self) -> Option<Password<'a>> {
        let min = self.num()?;
        self.skip_or_fail(b'-')?;
        let max = self.num()?;
        self.trim_spaces();
        let c = self.next()?;
        self.skip_or_fail(b':')?;
        self.trim_spaces()?;
        let pass = unsafe { std::str::from_utf8_unchecked(self.alphas()?) };

        Some(Password {
            policy: PasswordPolicy {
                min: min as u8,
                max: max as u8,
                c: c as char,
            },
            pass,
        })
    }
}
