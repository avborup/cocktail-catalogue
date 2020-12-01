use serde::{Serialize, Deserialize};

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Cocktail {
    pub id: i32,
    pub name: String,
    pub date_added: i32,
    pub source: Option<String>,
    pub author: String,

    #[serde(skip)]
    pub ingredients: Vec<CocktailIngredient>,
    #[serde(skip)]
    pub instructions: Vec<String>,
    #[serde(skip)]
    pub ratings: Vec<Rating>,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug)]
pub struct NewCocktail {
    name: String,
    source: Option<String>,
    author: String,
    ingredients: Vec<CocktailIngredientInput>,
    instructions: Vec<String>,
    ratings: Vec<NewRating>,
}

impl NewCocktail {
    pub fn to_cocktail(&self, id: i32, date_added: i32) -> Cocktail {
        Cocktail {
            id,
            date_added,
            name: self.name.clone(),
            source: self.source.clone(),
            author: self.author.clone(),
            ingredients: self.ingredients.clone().into_iter().map(|ing| ing.into()).collect(),
            instructions: self.instructions.clone(),
            ratings: self.ratings.clone().into_iter().map(|r| r.into()).collect(),
        }
    }
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct CocktailIngredient {
    label: String,
    amount: Option<f64>,
    unit: Option<String>,
    ingredient_type: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct CocktailIngredientInput {
    label: String,
    amount: Option<f64>,
    unit: Option<String>,
    ingredient_type: Option<String>,
}

impl From<CocktailIngredientInput> for CocktailIngredient {
    fn from(ing: CocktailIngredientInput) -> CocktailIngredient {
        CocktailIngredient {
            label: ing.label.clone(),
            amount: ing.amount,
            unit: ing.unit.clone(),
            ingredient_type: ing.ingredient_type.clone(),
        }
    }
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub rating: i32,
    pub author: String,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct NewRating {
    rating: i32,
    author: String,
}

impl From<NewRating> for Rating {
    fn from(rating: NewRating) -> Rating {
        Rating {
            rating: rating.rating,
            author: rating.author.clone(),
        }
    }
}
