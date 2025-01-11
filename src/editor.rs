use std::io::Error;
use core::cmp::min;
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Position, Size, Terminal};

const INIT_CHAR_TILDE: &str = "~";
const EXIT_MESSAGE: &str = "Goodbye!\r\n";
const STR_CR_LF: &str = "\r\n";
const STR_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const STR_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const STR_SPACE: &str = " ";


///
/// Location 構造体
/// キャレット位置を覚えておく
///
#[derive(Copy, Clone, Default)]
pub struct Location
{
	x: usize,
	y: usize,
}


///
/// Editor 構造体
///
#[derive(Default)]
pub struct Editor
{
	exit_editor: bool,
	location: Location,
}

///
/// Editor の実装部分
///
impl Editor
{
	/// derive(Default) と 特性を付与したため
	/// default を実装する必要がなくなった.
	// ///
	// /// default 定義
	// ///
	// pub const fn default() -> Self
	// {
	// 	Editor{exit_editor: false, location:}
	// }

	///
	/// 実行
	///
	pub fn run(&mut self)
	{
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
	fn repl(&mut self) -> Result<(), Error>
	{
		loop {
			self.refresh_screen()?;
			if self.exit_editor {
				// 終了のキー（Ctrl + Q）を検知したら終了する
				break;
			}
			let event = read()?;
			self.evaluate_event(&event)?;
		}
		Ok(())
	}

	///
	/// イベントの評価関数
	///
	fn evaluate_event(&mut self, event: &Event) -> Result<(), Error>
	{
		if let Key(KeyEvent {
					   code,
					   modifiers,
					   kind: KeyEventKind::Press, ..
		}) = event
		{
			match code {
				// Ctrl + "Q" で終了する．
				KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
					self.exit_editor = true;
				},
				// 十字キー等によるキャレットの移動
				KeyCode::Up
				| KeyCode::Down
				| KeyCode::Left
				| KeyCode::Right
				| KeyCode::PageUp
				| KeyCode::PageDown
				| KeyCode::Home
				| KeyCode::End => {
					self.move_point(*code, *modifiers)?;
				},
				_ => (),
			}
		}
		Ok(())
	}

	///
	/// キャレット位置を移動
	///
	fn move_point(&mut self, key_code: KeyCode, modifiers: KeyModifiers) -> Result<(), Error>
	{
		let Location{mut x, mut y} = self.location;
		let Size {height, width} = Terminal::size()?;
		let modify_ctrl = modifiers == KeyModifiers::CONTROL;
		match key_code {
			KeyCode::Up => { y = y.saturating_sub(1) },
			KeyCode::Down => { y = min(height.saturating_sub(1), y.saturating_add(1)) },
			KeyCode::Left => { x = x.saturating_sub(1) },
			KeyCode::Right => { x = min(width.saturating_add(1), x.saturating_add(1)) },
			KeyCode::PageUp => { y = 0 },
			KeyCode::PageDown => { y = height.saturating_sub(1) },
			KeyCode::Home => {
				x = 0;
				if modify_ctrl { y = 0 }
			},
			KeyCode::End => {
				x = width.saturating_sub(1);
				if modify_ctrl { y = height.saturating_sub(1) }
			},
			_ => (),
		}

		// 自身のキャレット位置を更新
		self.location = Location{x, y};
		Ok(())
	}

	///
	/// 画面の更新
	///
	fn refresh_screen(&self) -> Result<(), Error>
	{
		// キャレットの初期化
		Terminal::hide_caret()?;
		Terminal::move_caret_to(Position::default())?;

		if self.exit_editor {
			// 終了する
			Terminal::clear_screen()?;
			Terminal::print(EXIT_MESSAGE)?;
		} else {
			Self::draw_rows()?;
			Terminal::move_caret_to(Position {
				col: self.location.x,
				row: self.location.y,
			})?;
		}

		// キャレットを表示する
		Terminal::show_caret()?;
		Terminal::execute()?;
		Ok(())
	}

	///
	/// ウェルカムメッセージの描画
	///
	fn draw_welcome_message() -> Result<(), Error>
	{
		let mut welcome_msg = format!("{STR_PKG_NAME} editor -- version {STR_PKG_VERSION}");
		let width = Terminal::size()?.width as usize;
		let str_len = welcome_msg.len();
		let padding = (width - str_len) / 2;
		let spaces = STR_SPACE.repeat(padding - 1);
		welcome_msg = format!("~{spaces}{welcome_msg}");
		welcome_msg.truncate(width);
		Terminal::print(&welcome_msg)?;
		Ok(())
	}

	///
	/// ウェルカムメッセージの描画
	///
	fn draw_empty_row() -> Result<(), Error>
	{
		Terminal::print(INIT_CHAR_TILDE)?;
		Ok(())
	}

	///
	/// 行の描画
	///
	fn draw_rows() -> Result<(), Error>
	{
		let Size {height, ..} = Terminal::size()?;
		for cur_rows in 0..height {
			Terminal::clear_line()?;
			if cur_rows == height / 3 {
				Self::draw_welcome_message()?;
			} else {
				Self::draw_empty_row()?;
			}
			if cur_rows + 1 < height {
				Terminal::print(STR_CR_LF)?;
			}
		}
		Ok(())
	}
}
