use std::io;

struct TodoItem {
	body: String,
	done: char
}

impl TodoItem {
	fn new(todo: String) -> TodoItem {
		TodoItem {
			body: todo,
			done: 'n'
		}
	}
}

struct TodoList {
	items: Vec<TodoItem>
}

impl TodoList {
	fn new() -> TodoList {
		TodoList {
			items: Vec::new()
		}
	}

	fn print(&self) {
		println!("Total Todos:");
		for (index, item) in self.items.iter().enumerate() {
			println!("{} - {}", index, item.body);
		}
	}

	fn add(&mut self, item: String) {
		let to_item = TodoItem::new(item);
		self.items.push(to_item);
	}

	fn delete(&mut self, index: usize) {
		self.items.remove(index);
	}

	/*fn if_done(&mut self) {
		if self.items.done == 'n' {
			self.items.done = 'y';
		}
	}*/
}

// "Give first word or rest of the word"
fn gfworow(s: &str, rest: bool) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			if !rest {
				return &s[0..i];
			}
			return &s[i..];
		}
	}
	return " ";
}

fn main() {
	let mut todos = TodoList::new();

	println!("______Todo list______");
	println!("Commands:");
	println!("list new delete check exit\n");

	loop {
		let mut uinput = String::new();
		io::stdin().read_line(&mut uinput)
			.expect("Failed to read line");
		let value = gfworow(&uinput[..], false);
		match value {
			"new" => {
				let nv = String::from(gfworow(&uinput[..], true));
				todos.add(nv);
			},

			"delete" => {
				let nv = String::from(gfworow(&uinput[..], true));
				let nv = nv.trim().parse().expect("Enter an actual index");
				todos.delete(nv);
			},
			
			"list" => {
				if todos.items.len() >= 1 {
					todos.print();
				}
				else {
					println!("NO TODOS YET!");
				}
			}

			"check" => println!("Checking function not yet implemented"),

			"exit" => return,

			_ => continue,
		}
	}
}