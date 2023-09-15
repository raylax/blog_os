pub enum Key {}

pub const SCANCODE_ENTER: u8 = 0x1c;
pub const SCANCODE_BACKSPACE: u8 = 0x0e;

static SCANCODE_TABLE_1: [char; 12] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '='];
static SCANCODE_TABLE_2: [char; 12] = ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']'];
static SCANCODE_TABLE_3: [char; 11] = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\''];
static SCANCODE_TABLE_4: [char; 10] = ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'];

pub fn scancode_to_ascii(sc: u8) -> Option<char> {
    match sc {
        0x02..=0x0d => Some(SCANCODE_TABLE_1[(sc - 0x02) as usize]),
        0x10..=0x1b => Some(SCANCODE_TABLE_2[(sc - 0x10) as usize]),
        0x1e..=0x28 => Some(SCANCODE_TABLE_3[(sc - 0x1e) as usize]),
        0x2c..=0x35 => Some(SCANCODE_TABLE_4[(sc - 0x2c) as usize]),
        _ => None,
    }
}
