use std::io::stdout;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};

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
	pub fn default() -> Self {
		Editor{exit_editor: false}
	}

	///
	/// 実行
	///
	pub fn run(&mut self) {
		Self::initialize().unwrap();
		let result = self.repl();
		Self::terminate().unwrap();
		result.unwrap();
		// if let Err(err) = self.repl() {
		// 	panic!("{err:?}");
		// }
		// println!("GoodBye!\r\n");
	}

	///
	/// 初期化
	///
	fn initialize() -> Result<(), std::io::Error> {
		// raw モードを有効にする
		enable_raw_mode()?;
		Self::clear_screen()
	}

	///
	/// 終了処理
	///
	fn terminate() -> Result<(), std::io::Error> {
		// raw モードを無効にする
		disable_raw_mode()
	}

	///
	/// 画面のクリア
	///
	fn clear_screen() -> Result<(), std::io::Error> {
		let mut stdout = stdout();
		execute!(stdout, Clear(ClearType::All))
	}

	///
	/// 入力のループ処理
	///
	fn repl(&mut self) -> Result<(), std::io::Error> {
		loop {
			// キーイベントを取得する
			let event = read()?;

			// 取得したイベントを評価して
			// キーの内容に応じた処理を行う
			self.evaluate_event(&event);

			// 処理を行った後に画面を更新する
			self.refresh_screen()?;

			// 終了イベントを検知したら終了する
			if self.exit_editor {
				break;
			}
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
				Char('c') if *modifiers == KeyModifiers::CONTROL => {
					println!("Copy Text\r");
				}
				_ => (),
			}
		}
	}

	///
	/// 画面の更新
	///
	fn refresh_screen(&self) -> Result<(), std::io::Error> {
		if self.exit_editor {
			// 終了する
			Self::clear_screen()?;
			print!("Goodbye!\r\n");
		}
		Ok(())
	}
}
