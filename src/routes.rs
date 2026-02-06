use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::db::Database;
use crate::handlers::*;

pub fn create_router(db: Database) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Food item routes
        .route("/food-items", get(get_food_items))
        .route("/food-items", post(create_food_item))
        .route("/food-items/{id}", get(get_food_item))
        .route("/food-items/{id}", delete(delete_food_item))
        
        // Restaurant routes
        .route("/restaurants", get(get_restaurants))
        .route("/restaurants", post(create_restaurant))
        .route("/restaurants/{id}", get(get_restaurant))
        .route("/restaurants/{id}/menu", get(get_restaurant_menu))
        
        // Order routes
        .route("/orders", get(get_orders))
        .route("/orders", post(create_order))
        .route("/orders/{id}", get(get_order))
        .route("/orders/{id}/status", put(update_order_status))
        .route("/users/{user_id}/orders", get(get_user_orders))
        
        // User routes
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        
        // Add database state
        .with_state(db)
}