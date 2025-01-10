use crossterm::{queue, Command};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Print};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};
use core::fmt::Display;

///
/// Size 構造体
///
#[derive(Copy, Clone)]
pub struct Size
{
	pub width: u16,
	pub height: u16,
}

///
/// Position 構造体
///
#[derive(Copy, Clone)]
pub struct Position
{
	pub x: u16,
	pub y: u16,
}

/// //////////////////
/// Terminal 構造体
pub struct Terminal;

///
/// Terminal 実装部分
///
impl Terminal
{

	///
	/// 初期化処理
	///
	pub fn initialize()	-> Result<(), Error>
	{
		// raw モードを有効にする
		enable_raw_mode()?;
		Self::clear_screen()?;
		Self::move_cursor_to(Position {x: 0, y: 0})?;
		Self::execute()?;
		Ok(())
	}

	///
	/// 終了処理
	///
	pub fn terminate() -> Result<(), Error>
	{
		Self::execute()?;
		disable_raw_mode()?;
		Ok(())
	}

	///
	/// 画面の全体のクリア
	///
	pub fn clear_screen() -> Result<(), Error>
	{
		Self::queue_command(Clear(ClearType::All))?;
		Ok(())
	}

	///
	/// 一行クリア
	///
	pub fn clear_line() -> Result<(), Error>
	{
		Self::queue_command(Clear(ClearType::CurrentLine))?;
		Ok(())
	}

	///
	/// カーソルを指定した位置に移動する
	///
	pub fn move_cursor_to(position: Position) -> Result<(), Error>
	{
		Self::queue_command(MoveTo(position.x, position.y))?;
		Ok(())
	}

	///
	/// カーソルを隠す
	///
	pub fn hide_cursor() -> Result<(), Error>
	{
		Self::queue_command(Hide)?;
		Ok(())
	}

	///
	/// カーソルを表示する
	///
	pub fn show_cursor() -> Result<(), Error>
	{
		Self::queue_command(Show)?;
		Ok(())
	}

	///
	/// 文字をターミナルにプリントする
	///
	pub fn print(string: &str) -> Result<(), Error>
	{
		Self::queue_command(Print(string))?;
		Ok(())
	}

	///
	/// サイズを返す
	///
	pub fn size() -> Result<Size, Error>
	{
		let (width, height) = size()?;
		Ok(Size{width, height})
	}

	///
	/// 実行
	///
	pub fn execute() -> Result<(),Error>
	{
		stdout().flush()?;
		Ok(())
	}

	///
	/// 以下の書き方を簡素化するための実装
	/// queue!(stdout(), 任意のコマンド)?;
	///
	fn queue_command<T:Command>(command: T) -> Result<(), Error> {
		queue!(stdout(), command)?;
		Ok(())
	}
}
