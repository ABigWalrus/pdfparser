pub struct Token {
    toke_type: TokenType,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum TokenType {
    Obj,
    Endobj,
    EmptySpace,
    Raw,
}

pub fn tokenize(raw_data: Vec<u8>) {
    let n = raw_data.len();
    let mut i = 0;
    while i < n {
        let byte = raw_data[i];
        match byte {
            0x6f => {
                print!("o {:?}", TokenType::Obj);
            }

            0x62 => {
                print!("b {:?}", TokenType::Obj);
            }

            0x6a => {
                print!("j {:?}", TokenType::Obj);
            }

            0x65 => {
                print!("e {:?}", TokenType::Endobj);
            }

            0x6e => {
                print!("n {:?}", TokenType::Endobj);
            }

            0x64 => {
                print!("d {:?}", TokenType::Endobj);
            }
            _ => {
                print!("{:?}", TokenType::Raw);
            }
        }

        print!(" ");
        i = i + 1;
    }
}
