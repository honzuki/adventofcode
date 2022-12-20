#[derive(Debug, Clone, Copy)]
pub enum Cmd {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug, Clone, Copy)]
pub enum CmdErr {
    UnknownCmd,
}

impl Cmd {
    pub fn parse_line(line: &str) -> Result<Cmd, CmdErr> {
        let cmd = line.split_ascii_whitespace().clone().collect::<Vec<_>>();
        match &cmd[..] {
            &["U", count] => Ok(Cmd::Up(count.parse().unwrap())),
            &["R", count] => Ok(Cmd::Right(count.parse().unwrap())),
            &["L", count] => Ok(Cmd::Left(count.parse().unwrap())),
            &["D", count] => Ok(Cmd::Down(count.parse().unwrap())),
            _ => Err(CmdErr::UnknownCmd),
        }
    }
}
