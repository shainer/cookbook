// All printing commands go here.

extern crate rusqlite;
use rusqlite::Connection;

use recipes::Ingredient;
use recipes::Recipe;

fn find_recipe(connection: &Connection, search: &String) -> Result<Recipe, &'static str> {
    let rec_query = format!(
        "SELECT id,name,procedure FROM recipes WHERE name LIKE \"{}\"",
        search
    );

    let mut statement = connection.prepare(&rec_query).unwrap();
    let mut results_iter = statement
        .query_map(&[], |row| {
            Recipe {
                id: row.get(0),
                name: row.get(1),
                procedure: row.get(2),
                tags: Vec::new(),
                ingredients: Vec::new(),
            }
        })
        .unwrap();

    let mut recipe: Recipe = Recipe::new();
    match results_iter.nth(0) {
        Some(r) => {
            if r.is_err() {
                return Err("Internal error");
            }

            recipe = r.unwrap();
        }
        None => return Err("No recipe found."),
    }

    return Ok(recipe);
}

fn get_tags(connection: &Connection, id: i32) -> Result<Vec<String>, &'static str> {
    let tag_query = format!("SELECT tag FROM hasTag WHERE recipe={}", id);
    let mut tags: Vec<String> = Vec::new();

    let mut statement = connection.prepare(&tag_query).unwrap();
    let results_iter = statement.query_map(&[], |row| row.get(0)).unwrap();

    for res in results_iter {
        tags.push(res.unwrap());
    }

    Ok(tags)
}

fn get_ingredients(connection: &Connection, id: i32) -> Result<Vec<Ingredient>, &'static str> {
    let ing_query = format!(
        "SELECT ingredient,quantity,measure FROM hasIngredient WHERE recipe={}",
        id
    );
    let mut ingredients: Vec<Ingredient> = Vec::new();

    let mut statement = connection.prepare(&ing_query).unwrap();
    let results_iter = statement
        .query_map(&[], |row| {
            Ingredient {
                name: row.get(0),
                quantity: row.get(1),
                measure: row.get(2),
            }
        })
        .unwrap();

    for res in results_iter {
        ingredients.push(res.unwrap());
    }

    Ok(ingredients)
}

// TODO: handle returning more than one recipe for this search.
fn get_full_recipe(connection: &Connection, search: &String) -> Result<Recipe, &'static str> {
    match find_recipe(connection, search) {
        Ok(mut recipe) => {
            let tags = get_tags(connection, recipe.id);
            recipe.tags = tags.unwrap();

            let ingredients = get_ingredients(connection, recipe.id);
            recipe.ingredients = ingredients.unwrap();

            Ok(recipe)
        }
        Err(err) => return Err(err),
    }
}

pub fn print(connection: Connection, args: Vec<String>) {
    if args.len() < 3 {
        println!("Nothing to print; please specify a recipe name or part of it.");
        return;
    }

    match get_full_recipe(&connection, &args[2]) {
        Ok(recipe) => recipe.print(),
        Err(err) => println!("{}", err),
    }
}
