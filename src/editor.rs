use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


pub struct Editor {
	exit_editor: bool,
}

impl Editor {
	pub fn default() -> Self {
		Editor{exit_editor: false}
	}

	pub fn run(&mut self) {
		if let Err(err) = self.repl() {
			panic!("{err:?}");
		}
		println!("GoodBye!\r\n");
	}

	fn repl(&mut self) -> Result<(), std::io::Error> {
		// raw mode にする．
		enable_raw_mode()?;

		loop {
			if let Key(KeyEvent {code, modifiers, kind, state} ) =  read()? {
				if kind == KeyEventKind::Press {
					println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
				}
				match code {
					// Ctrl + "Q" で終了する．
					Char('q') if modifiers == KeyModifiers::CONTROL => {
						self.exit_editor = true;
					}
					_ => (),
				}
			}
			if self.exit_editor {
				break;
			}
		}

		// raw mode を解除する．
		disable_raw_mode()?;
		Ok(())
	}
}
