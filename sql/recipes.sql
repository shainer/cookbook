CREATE TABLE IF NOT EXISTS recipes (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` TEXT NOT NULL ,
  `procedure` TEXT NOT NULL);

CREATE INDEX `recipe_name` ON `recipes` (`name` ASC) ;

CREATE TABLE IF NOT EXISTS `hasTag` (
  `recipe` INTEGER NOT NULL,
  `tag` TEXT NOT NULL,
  CONSTRAINT `has_tags`
    FOREIGN KEY (`recipe`)
    REFERENCES `recipes` (`id` ));

CREATE TABLE IF NOT EXISTS `hasIngredient` (
  `recipe` INTEGER NOT NULL,
  `ingredient` TEXT NOT NULL,
  `quantity` INTEGER NOT NULL,
  `measure` TEXT NOT NULL,
  CONSTRAINT `has_ingredient`
    FOREIGN KEY (`recipe`)
    REFERENCES `recipes` (`id` ));
