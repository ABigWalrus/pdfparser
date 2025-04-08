pub struct Token {
    toke_type: TokenType,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum TokenType {
    Obj,
    EndObj,
    Raw,
    Null,
    Space,
    Tab,
    LF,
    FF,
    CR,
    Greater,
    Smaller,
    Slash,
    Backslash,
}

pub fn tokenize(raw_data: Vec<u8>) {
    let n = raw_data.len();
    let mut i = 0;
    while i < n {
        let byte = raw_data[i];
        match byte {
            0x6f => {
                if raw_data[i + 1] == 0x62 && raw_data[i + 2] == 0x6a {
                    print!("{:?}", TokenType::Obj);
                    i += 2;
                } else {
                    print!("{:?}", TokenType::Raw);
                }
            }

            0x65 => {
                if raw_data[i + 1] == 0x6e
                    && raw_data[i + 2] == 0x64
                    && raw_data[i + 3] == 0x6f
                    && raw_data[i + 4] == 0x62
                    && raw_data[i + 5] == 0x6a
                {
                    print!("{:?}", TokenType::EndObj);
                    i += 5;
                } else {
                    print!("{:?}", TokenType::Raw);
                }
            }

            0x3c => {
                print!("{:?}", TokenType::Smaller);
            }

            0x3e => {
                print!("{:?}", TokenType::Greater);
            }

            0x2f => {
                print!("{:?}", TokenType::Slash);
            }

            0x5c => {
                print!("{:?}", TokenType::Backslash);
            }

            0x00 => {
                print!("{:?}", TokenType::Null);
            }

            0x09 => {
                print!("{:?}", TokenType::Tab);
            }

            0x0A => {
                print!("{:?}", TokenType::LF);
            }

            0x0C => {
                print!("{:?}", TokenType::FF);
            }

            0x0D => {
                print!("{:?}", TokenType::CR);
            }

            0x20 => {
                print!("{:?}", TokenType::Space);
            }

            _ => {
                print!("{:?}", TokenType::Raw);
            }
        }

        print!(" ");
        i += 1;
    }
}
