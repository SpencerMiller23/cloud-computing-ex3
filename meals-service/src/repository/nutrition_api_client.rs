use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NutritionInformation {
    name: String,
    calories: f32,
    serving_size_g: f32,
    fat_total_g: f32,
    fat_saturated_g: f32,
    protein_g: f32,
    sodium_mg: f32,
    potassium_mg: i32,
    cholesterol_mg: i32,
    carbohydrates_total_g: f32,
    fiber_g: f32,
    sugar_g: f32
}

impl Default for NutritionInformation {
    fn default() -> Self {
        NutritionInformation {
            name: String::from(""),
            calories: 0.0,
            serving_size_g: 0.0,
            fat_total_g: 0.0,
            fat_saturated_g: 0.0,
            protein_g: 0.0,
            sodium_mg: 0.0,
            potassium_mg: 0,
            cholesterol_mg: 0,
            carbohydrates_total_g: 0.0,
            fiber_g: 0.0,
            sugar_g: 0.0    
        }
    }
}

impl NutritionInformation {
    pub fn get_calories(&self) -> f32 {
        self.calories
    }

    pub fn get_size(&self) -> f32 {
        self.serving_size_g
    }

    pub fn get_sodium(&self) -> f32 {
        self.sodium_mg
    }

    pub fn get_sugar(&self) -> f32 {
        self.sugar_g
    }
}

pub async fn get_nutrition_data(dish_name: String) -> Result<NutritionInformation, i32> {
    let request_url = format!("https://api.api-ninjas.com/v1/nutrition?query={dish_name}", dish_name = dish_name);

    let request = reqwest::Client::new()
        .get(&request_url)
        .header("X-Api-Key", "EpQWFI0Orj6GvxKtl4Xl0w==73SfSrj86CiQT2NK");
    
    let dishes: Vec<NutritionInformation>;

    let results = request.send().await;

    match results {
        Ok(response) => {
            match response.json::<Vec<NutritionInformation>>().await {
                Ok(d) => dishes = d,
                _ => return Err(-4)
            }
        },
        _ => return Err(-4)
    }

    let mut data = NutritionInformation { name: dish_name, ..Default::default() };

    for dish in dishes {
        data.calories += dish.calories;
        data.serving_size_g += dish.serving_size_g;
        data.fat_total_g += dish.fat_total_g;
        data.fat_saturated_g += dish.fat_saturated_g;
        data.protein_g += dish.protein_g;
        data.sodium_mg += dish.sodium_mg;
        data.potassium_mg += dish.potassium_mg;
        data.cholesterol_mg += dish.cholesterol_mg;
        data.carbohydrates_total_g += dish.carbohydrates_total_g;
        data.fiber_g += dish.fiber_g;
        data.sugar_g += dish.sugar_g;
    }

    Ok(data)
}