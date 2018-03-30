// Structures that describe recipes and its components.

pub struct Ingredient {
    pub name: String,
    pub quantity: i32,
    pub measure: String,
}

pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub procedure: String,
    pub tags: Vec<String>,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            id: -1,
            name: String::new(),
            procedure: String::new(),
            tags: Vec::new(),
            ingredients: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("=== Recipe for {} ===", self.name);
        if self.tags.len() > 0 {
            println!("Tags: {:?}", self.tags);
        }
        println!();

        if self.ingredients.len() > 0 {
            println!("Ingredients:");

            for i in &self.ingredients {
                println!("- {} {} of {}", i.quantity, i.measure, i.name);
            }
            println!();
        }

        println!("{}", self.procedure);
    }
}
