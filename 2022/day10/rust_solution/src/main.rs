use std::{env, error, fs, io};

mod cpu;
mod gif;

use cpu::CPU;

fn read_input() -> Result<String, io::Error> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

fn execute(input: &str, cpu: &mut CPU) -> Result<(), String> {
    for line in input.lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();
        match &tokens[..] {
            &["noop"] => cpu.noop(),
            &["addx", value] => cpu.add_x(
                value
                    .parse()
                    .map_err(|_| format!("can not parse {value}"))?,
            ),
            _ => return Err("unknown cmd".into()),
        };
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = read_input()?;

    let mut cpu = CPU::new();
    let gif_gen = gif::GifGen::new();
    cpu.hook_screen(gif_gen);
    execute(&input, &mut cpu)?;

    println!("part 1 result: {}", cpu.get_signal_strength());
    print!("{}", cpu.display_screen());

    Ok(())
}
