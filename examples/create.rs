use librde::desktopentry::{Type::Application, DesktopEntry};
use librde::serialize::Serialize;

fn main() {
	let my_entry = Serialize::new(&DesktopEntry {
		settype: Application,
		name: Some(String::from("My Shining Glitter Application")), // https://mlp.fandom.com/wiki/Starlight_Glimmer
		..Default::default()
	});

	println!("{}", my_entry);
}
