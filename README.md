# libreaddesktop
Serialize and Deserialize Desktop entries from [free desktop](https://specifications.freedesktop.org).

## TODO
- add tests.
- add test workflow.

## Referances.
- [Desktop Entry](https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html)

# Installation.
From git.
...

From crates.
...

# Usage.
Samples are under the [examples](examples/) directory, here's simple serialization example for fast start:

```rs
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

```

# Contributing.
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

# License
[GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
