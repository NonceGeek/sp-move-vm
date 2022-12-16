#![no_std]

pub fn stdlib_package() -> &'static [u8] {
    include_bytes!("../MoveStdlib.pac")
}

pub fn pont_stdlib_package() -> &'static [u8] {
    include_bytes!("../PontStdlib.pac")
}
