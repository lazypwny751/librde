#[derive(Debug, Default)]
pub enum Type {
	Application,
	Link,
	Directory,
	#[default]
	Unknown
}

// Ref: https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html
#[derive(Debug, Default)]
pub struct DesktopEntry {
	pub settype: Type,	// which is Type.
	pub version: Option<String>,
	pub name: Option<String>,
	pub genericname: Option<String>,
	pub nodisplay: Option<String>,
	pub comment: Option<String>,
	pub icon: Option<String>,
	pub hidden: Option<String>,
	pub onlyshowin: Option<Vec<String>>,
	pub notshowin: Option<Vec<String>>,
	pub dbusactivatable: Option<bool>,
	pub tryexec: Option<String>,
	pub exec: Option<String>,
	pub path: Option<String>,
	pub terminal: Option<bool>,
	pub actions: Option<Vec<String>>,
	pub mimetype: Option<Vec<String>>,
	pub categories: Option<Vec<String>>,
	pub implements: Option<Vec<String>>,
	pub keywords: Option<Vec<String>>,
	pub startupnotify: Option<bool>,
	pub startupwmclass: Option<String>,
	pub url: Option<String>,
	pub prefersnondefaultgpu: Option<bool>,
	pub singlemainwindow: Option<bool>
}
