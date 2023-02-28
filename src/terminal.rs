use termion::raw::RawTerminal;

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    stdout: RawTerminal<std::io::Stdout>,
}
