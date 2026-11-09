//! Standalone, testable gameplay primitives for Farmer's Delight on Horizon.
//! This crate does not yet register content or install hooks into Horizon.

pub mod cooking {
    /// Identifies an ingredient or output by its namespaced item id.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ingredient<'a> {
        pub item: &'a str,
        pub count: u32,
    }

    /// A deliberately small recipe representation. It is not a Horizon recipe API.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Recipe<'a> {
        pub ingredients: Vec<Ingredient<'a>>,
        pub output: Ingredient<'a>,
    }

    /// Counts matching item stacks; an ingredient may be split across stacks.
    /// This is a pure check: it does not mutate inventory.
    pub fn can_cook(recipe: &Recipe<'_>, inventory: &[Ingredient<'_>]) -> bool {
        if recipe
            .ingredients
            .iter()
            .any(|ingredient| ingredient.item.is_empty() || ingredient.count == 0)
        {
            return false;
        }

        for (index, required) in recipe.ingredients.iter().enumerate() {
            // Combine duplicate requirements for the same item exactly once.
            if recipe.ingredients[..index]
                .iter()
                .any(|previous| previous.item == required.item)
            {
                continue;
            }
            let total_required: u64 = recipe
                .ingredients
                .iter()
                .filter(|ingredient| ingredient.item == required.item)
                .map(|ingredient| u64::from(ingredient.count))
                .sum();
            let available: u64 = inventory
                .iter()
                .filter(|stack| stack.item == required.item)
                .map(|stack| u64::from(stack.count))
                .sum();
            if available < total_required {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn accepts_sufficient_ingredients_across_stacks() {
            let recipe = Recipe {
                ingredients: vec![
                    Ingredient {
                        item: "farmersdelight:tomato",
                        count: 2,
                    },
                    Ingredient {
                        item: "minecraft:bowl",
                        count: 1,
                    },
                ],
                output: Ingredient {
                    item: "farmersdelight:example_meal",
                    count: 1,
                },
            };
            let inventory = [
                Ingredient {
                    item: "farmersdelight:tomato",
                    count: 1,
                },
                Ingredient {
                    item: "farmersdelight:tomato",
                    count: 1,
                },
                Ingredient {
                    item: "minecraft:bowl",
                    count: 1,
                },
            ];
            assert!(can_cook(&recipe, &inventory));
        }

        #[test]
        fn rejects_missing_ingredient() {
            let recipe = Recipe {
                ingredients: vec![Ingredient {
                    item: "farmersdelight:tomato",
                    count: 2,
                }],
                output: Ingredient {
                    item: "farmersdelight:example_meal",
                    count: 1,
                },
            };
            assert!(!can_cook(
                &recipe,
                &[Ingredient {
                    item: "farmersdelight:tomato",
                    count: 1
                }]
            ));
        }

        #[test]
        fn combines_duplicate_requirements() {
            let recipe = Recipe {
                ingredients: vec![
                    Ingredient {
                        item: "farmersdelight:tomato",
                        count: 2,
                    },
                    Ingredient {
                        item: "farmersdelight:tomato",
                        count: 2,
                    },
                ],
                output: Ingredient {
                    item: "farmersdelight:example_meal",
                    count: 1,
                },
            };
            assert!(!can_cook(
                &recipe,
                &[Ingredient {
                    item: "farmersdelight:tomato",
                    count: 3
                }]
            ));
        }

        #[test]
        fn rejects_zero_quantity() {
            let recipe = Recipe {
                ingredients: vec![Ingredient {
                    item: "farmersdelight:tomato",
                    count: 0,
                }],
                output: Ingredient {
                    item: "farmersdelight:example_meal",
                    count: 1,
                },
            };
            assert!(!can_cook(&recipe, &[]));
        }
    }
}
