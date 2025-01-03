use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Terminal, Size, Position};

const INIT_CHAR_TILDE: &str = "~";
const EXIT_MESSAGE: &str = "Goodbye!\r\n";
const STR_CR_LF: &str = "\r\n";


///
/// Editor 構造体
///
pub struct Editor {
	exit_editor: bool,
}

///
/// Editor の実装部分
///
impl Editor {
	///
	/// default 定義
	///
	pub const fn default() -> Self {
		Editor{exit_editor: false}
	}

	///
	/// 実行
	///
	pub fn run(&mut self) {
		// 初期化
		Terminal::initialize().unwrap();

		// ループ処理開始
		let result = self.repl();
		Terminal::terminate().unwrap();
		result.unwrap();
	}

	///
	/// キー入力のループ処理
	///
	fn repl(&mut self) -> Result<(), Error> {
		loop {
			self.refresh_screen()?;
			if self.exit_editor {
				// 終了のキー（Ctrl + Q）を検知したら終了する
				break;
			}
			let event = read()?;
			self.evaluate_event(&event);
		}
		Ok(())
	}

	///
	/// イベントの評価関数
	///
	fn evaluate_event(&mut self, event: &Event) {
		if let Key(KeyEvent {
					   code, modifiers, .. }) = event
		{
			match code {
				// Ctrl + "Q" で終了する．
				Char('q') if *modifiers == KeyModifiers::CONTROL => {
					self.exit_editor = true;
				},
				// Char('c') if *modifiers == KeyModifiers::CONTROL => {
				// 	println!("Copy Text\r");
				// }
				_ => (),
			}
		}
	}

	///
	/// 画面の更新
	///
	fn refresh_screen(&self) -> Result<(), Error> {
		// カーソルを隠す
		Terminal::hide_cursor()?;

		if self.exit_editor {
			// 終了する
			Terminal::clear_screen()?;
			Terminal::print(EXIT_MESSAGE)?;
		} else {
			Self::draw_rows()?;
			Terminal::move_cursor_to(Position {x: 0, y: 0})?;
		}

		// カーソルを表示する
		Terminal::show_cursor()?;
		Terminal::execute()?;
		Ok(())
	}

	///
	/// 行の描画
	///
	fn draw_rows() -> Result<(), std::io::Error> {
		let Size {height, ..} = Terminal::size()?;
		for cur_rows in 0..height {
			Terminal::clear_line()?;
			Terminal::print(INIT_CHAR_TILDE)?;
			if cur_rows + 1 < height {
				Terminal::print(STR_CR_LF)?;
			}
		}
		Ok(())
	}
}
