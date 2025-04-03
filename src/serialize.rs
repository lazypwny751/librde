use crate::desktopentry::DesktopEntry;

pub trait Serialize {
	fn new(&self) -> String;
}

impl Serialize for DesktopEntry {
	fn new(&self) -> String {
		format!(
			r#"[Desktop Entry]
{}"#, "hello")
	}
}
