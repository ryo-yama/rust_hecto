use super::terminal::{Size, Terminal};
use std::io::Error;

const STR_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const STR_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const INIT_CHAR_TILDE: &str = "~";
const STR_SPACE: &str = " ";
const STR_CR_LF: &str = "\r\n";
const STR_HELLO_WORLD: &str = "Hello, World!\r\n";


///
/// View 構造体
///
pub struct View;

///
/// View 構造体の実装部分
///
impl View
{
	///
	/// 描画処理
	///
	pub fn render() -> Result<(), Error>
	{
		// 1行目に "Hello, World!" を出力する．
		Terminal::clear_line()?;
		Terminal::print(STR_HELLO_WORLD)?;

		let Size {height, ..} = Terminal::size()?;
		for cur_row in 1..height {
			Terminal::clear_line()?;
			// 整数
			#[allow(clippy::integer_division)]
			if cur_row == height / 3 {
				Self::draw_welcome_message()?;
			} else {
				Self::draw_empty_row()?;
			}
			if cur_row + 1 < height {
				Terminal::print(STR_CR_LF)?;
			}
		}
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
}