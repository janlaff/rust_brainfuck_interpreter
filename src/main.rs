use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

struct BrainfuckContext<'a> {
    cells: [u8; 30000],
    code: &'a [u8],
    cell_index: usize,
    code_index: usize
}

impl<'a> BrainfuckContext<'a> {
    fn new(code: &'a String) -> Self {
        BrainfuckContext { cells: [0; 30000], code: code.as_bytes(), cell_index: 0, code_index: 0 }
    }

    fn next_cell(&mut self) {
        self.cell_index += 1;
        self.code_index += 1;
    }

    fn prev_cell(&mut self) {
        self.cell_index -= 1;
        self.code_index += 1;
    }

    fn inc_cell(&mut self) {
        // Simulate overflow
        if self.cells[self.cell_index] == 255 {
            self.cells[self.cell_index] = 0;
        } else {
            self.cells[self.cell_index] += 1;
        }

        self.code_index += 1;
    }

    fn dec_cell(&mut self) {
        self.cells[self.cell_index] -= 1;
        self.code_index += 1;
    }

    fn read_cell(&mut self) {
        let input : Option<u8> = io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());

        self.cells[self.cell_index] = match input {
            Some(i) => i,
            None => panic!("Failed to read char")
        };

        self.code_index += 1;
    }

    fn put_cell(&mut self) {
        io::stdout().write(&[self.cells[self.cell_index]]);
        io::stdout().flush();
        self.code_index += 1;
    }

    fn bracket_open_check(&mut self) {
        if self.cells[self.cell_index] == 0 {
            let mut brackets = 1;

            while brackets > 0 {
                self.code_index += 1;

                match self.code[self.code_index] as char {
                    '[' => brackets += 1,
                    ']' => brackets -= 1,
                    _ => ()
                }
            }
        }

        self.code_index += 1;
    }

    fn bracket_close_check(&mut self) {
        if self.cells[self.cell_index] != 0 {
            let mut brackets = 1;

            while brackets > 0 {
                self.code_index -= 1;

                match self.code[self.code_index] as char {
                    ']' => brackets += 1,
                    '[' => brackets -= 1,
                    _ => ()
                }
            }
        }

        self.code_index += 1;
    }

    fn run(&mut self) {
        while self.code_index < self.code.len() {
            match self.code[self.code_index] as char {
                '+' => self.inc_cell(),
                '-' => self.dec_cell(),
                '>' => self.next_cell(),
                '<' => self.prev_cell(),
                '.' => self.put_cell(),
                ',' => self.read_cell(),
                '[' => self.bracket_open_check(),
                ']' => self.bracket_close_check(),
                _ => self.code_index += 1
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./RustBrainfuckInterpreter [-p <PROGRAM>]/[<FILE>]");
        process::exit(1);
    }

    if args[1] == "-p" && args.len() >= 2 {
        let mut ctx = BrainfuckContext::new(&args[2]);
        ctx.run();
    } else {
        let program = match fs::read_to_string(&args[1]) {
            Ok(contents) => contents,
            Err(_) => {
                println!("Failed to read file");
                process::exit(1);
            }
        };

        let mut ctx = BrainfuckContext::new(&program);
        ctx.run();

    }
}
