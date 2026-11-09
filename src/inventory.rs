//! Inventory updates for cooking recipes, independent of Horizon hooks.

use crate::cooking::{Ingredient, Recipe, can_cook};

/// A successful cooking operation consumes inputs and produces the recipe output.
/// Returns `None` without changing the original inventory when inputs are missing.
pub fn cook<'a>(
    recipe: &Recipe<'a>,
    inventory: &[Ingredient<'a>],
) -> Option<(Vec<Ingredient<'a>>, Ingredient<'a>)> {
    if !can_cook(recipe, inventory) || recipe.output.item.is_empty() || recipe.output.count == 0 {
        return None;
    }

    let mut remaining = inventory.to_vec();
    for ingredient in &recipe.ingredients {
        let mut needed = ingredient.count;
        for stack in &mut remaining {
            if stack.item != ingredient.item {
                continue;
            }
            let consumed = stack.count.min(needed);
            stack.count -= consumed;
            needed -= consumed;
            if needed == 0 {
                break;
            }
        }
        debug_assert_eq!(needed, 0);
    }
    remaining.retain(|stack| stack.count > 0);
    Some((remaining, recipe.output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumes_multiple_stacks_and_preserves_unrelated_items() {
        let recipe = Recipe {
            ingredients: vec![Ingredient {
                item: "farmersdelight:tomato",
                count: 3,
            }],
            output: Ingredient {
                item: "farmersdelight:meal",
                count: 1,
            },
        };
        let inventory = [
            Ingredient {
                item: "farmersdelight:tomato",
                count: 2,
            },
            Ingredient {
                item: "farmersdelight:tomato",
                count: 2,
            },
            Ingredient {
                item: "minecraft:carrot",
                count: 5,
            },
        ];
        let (remaining, output) = cook(&recipe, &inventory).unwrap();
        assert_eq!(output, recipe.output);
        assert_eq!(
            remaining,
            vec![
                Ingredient {
                    item: "farmersdelight:tomato",
                    count: 1,
                },
                Ingredient {
                    item: "minecraft:carrot",
                    count: 5,
                },
            ]
        );
        assert_eq!(inventory[0].count, 2);
    }

    #[test]
    fn rejects_insufficient_inputs_without_mutation() {
        let recipe = Recipe {
            ingredients: vec![Ingredient {
                item: "farmersdelight:tomato",
                count: 2,
            }],
            output: Ingredient {
                item: "farmersdelight:meal",
                count: 1,
            },
        };
        let inventory = [Ingredient {
            item: "farmersdelight:tomato",
            count: 1,
        }];
        assert_eq!(cook(&recipe, &inventory), None);
        assert_eq!(inventory[0].count, 1);
    }

    #[test]
    fn consumes_duplicate_recipe_requirements() {
        let recipe = Recipe {
            ingredients: vec![
                Ingredient {
                    item: "farmersdelight:tomato",
                    count: 1,
                },
                Ingredient {
                    item: "farmersdelight:tomato",
                    count: 2,
                },
            ],
            output: Ingredient {
                item: "farmersdelight:meal",
                count: 1,
            },
        };
        let inventory = [Ingredient {
            item: "farmersdelight:tomato",
            count: 3,
        }];
        let (remaining, _) = cook(&recipe, &inventory).unwrap();
        assert!(remaining.is_empty());
    }
}
