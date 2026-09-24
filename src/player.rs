struct Item {
    cost: u32,
    name: String,
}

struct ItemStack {
    item: Item,
    quantity: u32,
}

struct Inventory {
    size: u32,
    items: Vec<ItemStack>,
}

impl Inventory {
    fn new(size: u32) -> Inventory {
        Inventory {
            size,
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, cost: u32, name: String) {
		for stack in &mut self.items {
			if stack.item.name == name {
				stack.quantity += 1;
				return;
			}
		}
		if self.items.len() < self.size as usize {
			self.items.push(ItemStack {
				item: Item { cost, name },
				quantity: 1,
			});
		}
	}

	fn remove_item(&mut self, name: String) {
		let mut removed: i32 = -1;
		for (index, stack) in self.items.iter_mut().enumerate() {
			if stack.item.name == name {
				if stack.quantity > 0 {
					stack.quantity -= 1;
				}
				if stack.quantity == 0 {
					removed = index as i32;
				}
				break;
			}
		}
		if removed != -1 {
			self.items.remove(removed as usize);
		}
	}
}

pub struct Entity {
    nom: String,
    inventory: Inventory,
    location: String,
	coord: (f32, f32),
	flip: bool,
}

impl Entity {
    pub fn nouvelle(nom: String, coord: (f32, f32)) -> Entity {
        Entity {
            nom,
            inventory: Inventory::new(10),
            location: String::from("Marché"),
			coord,
			flip: false,
        }
    }

	pub fn get_coord(&self) -> (f32, f32){
        self.coord
    }

	pub fn get_flip(&self) -> bool {
		self.flip
	}

	pub fn add_inventory(&mut self, cost: u32, name: String) {
		self.inventory.add_item(cost, name);
	}

	pub fn drop_inventory (&mut self, name: String) {
		self.inventory.remove_item(name);
	}

    fn se_presenter(&self) {
        println!(
            "Je m'appelle {} et je suis au {}",
            self.nom, self.location,
        );
    }

	pub fn get_inventory(&self) -> Vec<(String, u32, u32)> {
		let mut output = vec![];
		for x in &self.inventory.items {
			output.push((x.item.name.clone(), x.quantity, x.item.cost));
		}
		output
	}

	pub fn moove(&mut self, movement: &str) {
		let (x2, y2) = match movement {
			"N" => (0.0, -0.1),
			"S" => (0.0, 0.1),
			"E" => {
				self.flip = false;
				(0.1, 0.0)
			}
			"W" => {
				self.flip = true;
				(-0.1, 0.0)
			}
			_ => (0.0, 0.0),
		};
		self.coord.0 += x2;
		self.coord.1 += y2;
	}
}