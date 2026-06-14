

enum Tiledata {
    Glyph(usize),
    Data(Vec<u8>)
}

pub enum FeedResult {
    Byte(u8),
    Bytes(Vec<u8>),
    Glyph(usize),
    Unknown
}

#[derive(Debug)]
enum ParserState {
    Outside,
    InsidePossibleFirstEsc,
    InsideBody,
    InsidePossibleEndingEsc,

}

pub struct Parser {
    state: ParserState,
    buffer: Vec<u8>
}

impl Parser {

    pub fn new() -> Self {
        Parser {
            state: ParserState::Outside,
            buffer: Vec::new()
        }
    }

    fn not_us(&mut self) -> FeedResult {
        self.state = ParserState::Outside;
        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut self.buffer);
        return FeedResult::Bytes(buffer);
    }

    pub fn feed(&mut self, byte: u8) -> FeedResult {
        // eprintln!("Buffer: {:?}", self.buffer);
        // eprintln!("State: {:?}", self.state);
        // eprintln!("Feeding: {:?}", byte);
        match self.state {
            ParserState::Outside => {
                if byte != b'\x1B'  {
                    return FeedResult::Byte(byte);
                } else {
                    self.buffer.push(byte);
                    self.state = ParserState::InsidePossibleFirstEsc;
                    return FeedResult::Unknown;
                }
            },
            ParserState::InsidePossibleFirstEsc => {
                self.buffer.push(byte);
                if byte == b'z' {
                    if self.buffer.get(4) != Some(&b'0') {
                        // Some vt_tiledata code we don't currently recognize
                        return self.not_us();
                    }
                    self.state = ParserState::InsideBody;
                    return FeedResult::Unknown;
                } else if b'\x40' <= byte && byte <= b'\x7F' && self.buffer.len() != 2 {
                    // This wasn't a NetHack tiledata
                    return self.not_us();
                } else {
                    return FeedResult::Unknown;
                }
            },
            ParserState::InsideBody => {
                self.buffer.push(byte);
                if byte == b'\x1B' {
                    self.state = ParserState::InsidePossibleEndingEsc;
                }
                return FeedResult::Unknown;
            },
            ParserState::InsidePossibleEndingEsc => {
                self.buffer.push(byte);
                if byte == b'z' {
                    let num_as_bytes = self.buffer.iter().skip(6).take_while(|b| b != &&b';').copied().collect::<Vec<u8>>();
                    let num_as_string = String::from_utf8_lossy(num_as_bytes.as_slice());
                    let num_result  = num_as_string.parse::<usize>();
                    match num_result {
                        Ok(num) => {
                            self.buffer.clear();
                            self.state = ParserState::Outside;
                            return FeedResult::Glyph(num);
                        },
                        Err(_) => {
                            // eprintln!("Error parsing number? {:?}", num_as_string);
                            // eprintln!("Buffer was {:?}", self.buffer);
                            return self.not_us();
                        }
                    }
                } else {
                    return FeedResult::Unknown; // Could be sequence we don't care about
                }
            }

        }
    }
}