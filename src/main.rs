extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color;
use std::io::{Write, stdout, stdin};

fn main() {
		let mut x:u16 = 1;
		let mut y:u16 = 1; 
		// Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    write!(stdout, "{}{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide

           ).unwrap();
    // Flush stdout (i.e. make the output appear).
    draw_user(&mut x, &mut y);
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}{}",
        	termion::cursor::Goto(1, 1),
        	termion::clear::CurrentLine,
        	termion::clear::BeforeCursor).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            Key::Left      => {
            	x -=1;
            	draw_user(&mut x,&mut y);
            },
            Key::Right     => {
            	x +=1;
            	draw_user(&mut x,&mut y);
            },
            Key::Up        => {
            	y -=1;
            	draw_user(&mut x,&mut y);
            },
            Key::Down      => {
            	y +=1;
            	draw_user(&mut x,&mut y);
            },
            _ => {},
        }
				stdout.flush().unwrap();
    }
    // Show the cursor again before we exit.
    write!(stdout, "{}{}", termion::cursor::Show, termion::clear::AfterCursor).unwrap();
    // Flush again.
}

fn draw_user(x: &mut u16, y: &mut u16) {
	let termsize = termion::terminal_size().ok();
  let termwidth = termsize.map(|(w,_)| w-2).unwrap();
	let termheight = termsize.map(|(_,h)| h-1).unwrap();
	if *x<1 {
		*x=1;
	}
	if *x>termwidth {
		*x=termwidth;
	}
	if *y<1 {
		*y=1;
	}
	if *y> termheight {
		*y=termheight;
	}
	for h in 1..termheight+1 {
		for w in 1..termwidth+1 {
			//print!("w:{}",w);
			if h == *y && w == *x {
				print!("{}☻{}",color::Fg(color::LightGreen),color::Fg(color::Reset));
			} else {
				print!(".");
			}
		}
		println!("\r");
	}
}
