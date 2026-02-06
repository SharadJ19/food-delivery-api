use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::*;

// In-memory database structure
#[derive(Debug, Clone)]
pub struct Database {
    pub food_items: Arc<RwLock<HashMap<Uuid, FoodItem>>>,
    pub restaurants: Arc<RwLock<HashMap<Uuid, Restaurant>>>,
    pub orders: Arc<RwLock<HashMap<Uuid, Order>>>,
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl Database {
    pub fn new() -> Self {
        // Create some initial data
        let mut food_items = HashMap::new();
        let mut restaurants = HashMap::new();
        let mut users = HashMap::new();
        
        // Create sample user
        let user = User::new(
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "1234567890".to_string(),
            "123 Main St".to_string(),
        );
        users.insert(user.id, user);
        
        // Create sample restaurant
        let restaurant = Restaurant::new(
            "Pizza Palace".to_string(),
            "Best pizza in town".to_string(),
            "456 Food Street".to_string(),
            "9876543210".to_string(),
            "30-45 mins".to_string(),
            2.99,
            15.0,
            Some("https://example.com/pizza.jpg".to_string()),
        );
        restaurants.insert(restaurant.id, restaurant.clone());
        
        // Create sample food items
        let pizza = FoodItem::new(
            "Margherita Pizza".to_string(),
            "Classic tomato and mozzarella".to_string(),
            12.99,
            "Pizza".to_string(),
            Some("https://example.com/margherita.jpg".to_string()),
        );
        food_items.insert(pizza.id, pizza);
        
        let pasta = FoodItem::new(
            "Spaghetti Carbonara".to_string(),
            "Creamy pasta with bacon".to_string(),
            14.99,
            "Pasta".to_string(),
            Some("https://example.com/carbonara.jpg".to_string()),
        );
        food_items.insert(pasta.id, pasta);
        
        Self {
            food_items: Arc::new(RwLock::new(food_items)),
            restaurants: Arc::new(RwLock::new(restaurants)),
            orders: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(users)),
        }
    }
}