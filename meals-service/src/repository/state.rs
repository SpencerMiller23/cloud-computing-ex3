use std::{
    sync::Mutex,
    collections::HashMap, i32
};

use serde::{Serialize, Deserialize};

use crate::{repository::nutrition_api_client::get_nutrition_data, api::meal};

use super::nutrition_api_client::NutritionInformation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    ID: i32,
    name: String,
    cal: f32,
    size: f32,
    sodium: f32,
    sugar: f32
}

impl Dish {
    fn new(id: i32, name: String, data: NutritionInformation) -> Dish {
        Dish {
            name: name,
            ID: id,
            cal: data.get_calories(),
            size: data.get_size(),
            sodium: data.get_sodium(),
            sugar: data.get_sugar()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Meal {
    ID: i32,
    name: String,
    appetizer: Option<i32>,
    main: Option<i32>,
    dessert: Option<i32>,
    cal: f32,
    sodium: f32,
    sugar: f32
}

impl Meal {
    fn new(id: &i32, name: String, appetizer: &Dish, main: &Dish, dessert: &Dish) -> Meal {
        Meal {
            ID: *id,
            name: name,
            appetizer: Some(appetizer.ID),
            main: Some(main.ID),
            dessert: Some(dessert.ID),
            cal: appetizer.cal + main.cal + dessert.cal,
            sodium: appetizer.sodium + main.sodium + dessert.sodium,
            sugar: appetizer.sugar + main.sugar + dessert.sugar
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    dish_counter: Mutex<i32>,
    dishes: Mutex<HashMap<i32, Dish>>,
    dish_ids: Mutex<HashMap<String, i32>>,
    meal_counter: Mutex<i32>,
    meals: Mutex<HashMap<i32, Meal>>,
    meal_ids: Mutex<HashMap<String, i32>>
}

impl AppState {
    pub fn new() -> AppState {
        return AppState {
            dish_counter: Mutex::new(0),
            dishes: Mutex::new(HashMap::new()),
            dish_ids: Mutex::new(HashMap::new()),
            meal_counter: Mutex::new(0),
            meals: Mutex::new(HashMap::new()),
            meal_ids: Mutex::new(HashMap::new())
        }
    }

    pub fn rebuild(&self) -> i32 { // Delete
        let mut counter = self.dish_counter.lock().unwrap();
        *counter = 3;

        let pasta = Dish {
            ID: 1,
            name: String::from("pasta"),
            cal: 500.0,
            size: 150.0,
            sodium: 12.0,
            sugar: 1.0
        };

        let focaccia = Dish {
            ID: 2,
            name: String::from("focaccia"),
            cal: 251.4,
            size: 100.0,
            sodium: 570.0,
            sugar: 1.8
        };

        let chicken_soup = Dish {
            ID: 3,
            name: String::from("chicken soup"),
            cal: 33.2,
            size: 100.0,
            sodium: 230.0,
            sugar: 1.0
        };
        
        let salad = Dish {
            ID: 4,
            name: String::from("salad"),
            cal: 28.2,
            size: 100.0,
            sodium: 78.2,
            sugar: 6.0
        };

        let mut dishes = self.dishes.lock().unwrap();
        let mut dish_ids = self.dish_ids.lock().unwrap();
        
        dishes.insert(1, pasta);
        dish_ids.insert(String::from("pasta"), 1);
        
        dishes.insert(2, focaccia);
        dish_ids.insert(String::from("focaccia"), 2);
        
        dishes.insert(3, chicken_soup);
        dish_ids.insert(String::from("chicken soup"), 3);
        
        dishes.insert(4, salad);
        dish_ids.insert(String::from("salad"), 4);

        4
    }

    fn increment_counter(&self, counter: &Mutex<i32>) -> i32 {
        let mut ctr = counter.lock().unwrap();
        *ctr += 1;

        let val = ctr.clone();

        return val
    }

    pub async fn create_dish(&self, name: String) -> Result<i32, i32> {
        let dish = self.get_dish_by_name(name.clone());

        match dish {
            Ok(_) => Err(-2), // Dish already exists
            _ => {
                let nutrition_data = get_nutrition_data(name.clone()).await;

                match nutrition_data {
                    Ok(data) => {
                        let dish_id = self.increment_counter(&self.dish_counter);

                        let dish = Dish::new(dish_id, name.clone(), data);

                        let mut dishes = self.dishes.lock().unwrap();
                        let mut dish_ids = self.dish_ids.lock().unwrap();
                        
                        dishes.insert(dish_id, dish);
                        dish_ids.insert(name.clone(), dish_id);

                        Ok(dish_id)
                    },
                    Err(err) => Err(err),
                }
            },
        }
    }

    pub fn get_dish_by_name(&self, name: String) -> Result<Dish, i32> {
        let dish_ids = self.dish_ids.lock().unwrap();

        let dish_id = dish_ids.get(&name);

        match dish_id {
            Some(id) => return self.get_dish_by_id(*id),
            None => Err(-5)
        }
    }

    pub fn get_dish_by_id(&self, id: i32) -> Result<Dish, i32> {
        let dishes = self.dishes.lock().unwrap();

        let dish = dishes.get(&id);

        match dish {
            Some(d) => Ok(d.clone()),
            None => Err(-5)
        }
    }

    pub fn get_dishes(&self) -> HashMap<i32, Dish> {
        let dishes = self.dishes.lock().unwrap();

        let dishes_copy = dishes.clone();

        return dishes_copy
    }

    pub fn delete_dish_by_id(&self, id: i32) -> Result<i32, i32> {
        let mut dishes = self.dishes.lock().unwrap();

        match dishes.remove(&id) {
            Some(data) => {
                let mut dish_ids = self.dish_ids.lock().unwrap();

                match dish_ids.remove(&data.name) {
                    Some(_) => return Ok(id),
                    None => panic!("Dish not found")
                }
            },
            None => Err(-5)
        }
    }

    pub fn delete_dish_by_name(&self, name: String) -> Result<i32, i32> {
        let mut dish_ids = self.dish_ids.lock().unwrap();

        match dish_ids.remove(&name) {
            Some(id) => {
                let mut dishes = self.dishes.lock().unwrap();

                match dishes.remove(&id) {
                    Some(_) => return Ok(id),
                    None => panic!("Dish not found")
                }
            },
            None => Err(-5)
        }
    }

    pub fn create_meal(&self, name: String, appetizer_id: &i32, main_id: &i32, dessert_id: &i32) -> Result<i32, i32> {
        let appetizer: Dish;
        let main: Dish;
        let dessert: Dish;

        let meal = self.get_meal_by_name(&name);

        match meal {
            Ok(_) => return Err(-2),
            _ => {}
        }
        
        match self.get_dish_by_id(*appetizer_id) {
            Ok(dish) => appetizer = dish,
            _ => return Err(-6)
        }
        
        match self.get_dish_by_id(*main_id) {
            Ok(dish) => main = dish,
            _ => return Err(-6)
        }

        match self.get_dish_by_id(*dessert_id) {
            Ok(dish) => dessert = dish,
            Err(_) => return Err(-6)
        }

        let meal_id = self.increment_counter(&self.meal_counter);

        let meal = Meal::new(&meal_id, name.clone(), &appetizer, &main, &dessert);

        let mut meals = self.meals.lock().unwrap();
        let mut meal_ids = self.meal_ids.lock().unwrap();

        meals.insert(meal_id, meal);
        meal_ids.insert(name, meal_id);

        Ok(meal_id)
    }

    pub fn get_meal_by_name(&self, name: &String) -> Result<Meal, i32> {
        let meal_ids = self.meal_ids.lock().unwrap();

        let meal_id = meal_ids.get(name);

        match meal_id {
            Some(id) => self.get_meal_by_id(*id),
            None => Err(-5)
        }
    }

    pub fn get_meal_by_id(&self, id: i32) -> Result<Meal, i32> {
        let meals = self.meals.lock().unwrap();

        let meal = meals.get(&id);

        match meal {
            Some(m) => Ok(m.clone()),
            None => Err(-5)
        }
    }

    pub fn get_meals(&self) -> HashMap<i32, Meal> {
        let meals = self.meals.lock().unwrap();

        let meals_copy = meals.clone();

        return meals_copy
    }

    pub fn delete_meal_by_id(&self, id: &i32) -> Result<i32, i32> {
        let mut meals = self.meals.lock().unwrap();

        match meals.remove(id) {
            Some(meal) => {
                let mut meal_ids = self.meal_ids.lock().unwrap();

                match meal_ids.remove(&meal.name) {
                    Some(_) => Ok(*id),
                    None => panic!("Meal not found")
                }
            },
            None => Err(-5)
        }
    }

    pub fn delete_meal_by_name(&self, name: &String) -> Result<i32, i32> {
        let mut meal_ids = self.meal_ids.lock().unwrap();
        
        match meal_ids.remove(name) {
            Some(id) => {
                let mut meals = self.meals.lock().unwrap();

                match meals.remove(&id) {
                    Some(_) => Ok(id),
                    None => panic!("Meal not found")
                }
            },
            None => Err(-5)
        }
    }

    pub fn update_meal(&self, id: &i32, name: &String, appetizer_id: &i32, main_id: &i32, dessert_id: &i32) -> Result<i32, i32> {
        let mut meals = self.meals.lock().unwrap();

        let meal = meals.get(id);

        match meal {
            Some(data) => {
                if *name != data.name {
                    let new_meal_id = self.get_meal_by_name(name);

                    match new_meal_id {
                        Ok(_) => return Err(-2),
                        _ => {}
                    }

                    let mut meal_ids = self.meal_ids.lock().unwrap();

                    let res = meal_ids.remove(&data.name);

                    match res {
                        Some(id) => {
                            meal_ids.insert(name.clone(), id);
                        },
                        None => panic!("Meal not found")
                    }
                }

                let appetizer: Dish;
                let main: Dish;
                let dessert: Dish;

                match self.get_dish_by_id(*appetizer_id) {
                    Ok(dish) => appetizer = dish,
                    Err(_) => return Err(-6)
                }

                match self.get_dish_by_id(*main_id) {
                    Ok(dish) => main = dish,
                    Err(_) => return Err(-6)
                }

                match self.get_dish_by_id(*dessert_id) {
                    Ok(dish) => dessert = dish,
                    Err(_) => return Err(-6)
                }

                let new_meal = Meal::new(id, name.clone(), &appetizer, &main, &dessert);

                meals.insert(*id, new_meal);

                Ok(*id)
            },
            None => Err(-5)
        }
    }
}