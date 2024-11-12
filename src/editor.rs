use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


pub struct Editor {

}

impl Editor {
	pub fn default() -> Self {
		Editor{}
	}

	pub fn run(&self) {
		// raw mode にする．
		enable_raw_mode().unwrap();

		loop {
			match read () {
				Ok(Key(event)) => {
					println!("{:?} \r", event);
					match event.code {
						Char(input_char) => {
							if  'q' == input_char {
								break;
							}
						},
						_ => (),
					}
				},
				Err(err) => println!("Error: {}", err),
				_ => ()
			}
		}

		// raw mode を解除する．
		disable_raw_mode().unwrap();
	}
}
