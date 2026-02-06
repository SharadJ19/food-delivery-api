use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::Database;
use crate::models::*;

// ========== FOOD ITEM HANDLERS ==========

/// Get all food items
pub async fn get_food_items(
    State(db): State<Database>,
) -> Result<Json<Vec<FoodItem>>, (StatusCode, String)> {
    let food_items = db.food_items.read().await;
    let items: Vec<FoodItem> = food_items.values().cloned().collect();
    Ok(Json(items))
}

/// Get food item by ID
pub async fn get_food_item(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<Json<FoodItem>, (StatusCode, String)> {
    let food_items = db.food_items.read().await;
    match food_items.get(&id) {
        Some(item) => Ok(Json(item.clone())),
        None => Err((StatusCode::NOT_FOUND, "Food item not found".to_string())),
    }
}

/// Create new food item
pub async fn create_food_item(
    State(db): State<Database>,
    Json(payload): Json<CreateFoodItemRequest>,
) -> Result<Json<FoodItem>, (StatusCode, String)> {
    let new_item = FoodItem::new(
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.image_url,
    );
    
    let mut food_items = db.food_items.write().await;
    food_items.insert(new_item.id, new_item.clone());
    
    Ok(Json(new_item))
}

/// Delete food item
pub async fn delete_food_item(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut food_items = db.food_items.write().await;
    if food_items.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Food item not found".to_string()))
    }
}

// ========== RESTAURANT HANDLERS ==========

/// Get all restaurants
pub async fn get_restaurants(
    State(db): State<Database>,
) -> Result<Json<Vec<Restaurant>>, (StatusCode, String)> {
    let restaurants = db.restaurants.read().await;
    let items: Vec<Restaurant> = restaurants.values().cloned().collect();
    Ok(Json(items))
}

/// Get restaurant by ID
pub async fn get_restaurant(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<Json<Restaurant>, (StatusCode, String)> {
    let restaurants = db.restaurants.read().await;
    match restaurants.get(&id) {
        Some(restaurant) => Ok(Json(restaurant.clone())),
        None => Err((StatusCode::NOT_FOUND, "Restaurant not found".to_string())),
    }
}

/// Create new restaurant
pub async fn create_restaurant(
    State(db): State<Database>,
    Json(payload): Json<CreateRestaurantRequest>,
) -> Result<Json<Restaurant>, (StatusCode, String)> {
    let new_restaurant = Restaurant::new(
        payload.name,
        payload.description,
        payload.address,
        payload.phone,
        payload.delivery_time,
        payload.delivery_fee,
        payload.minimum_order,
        payload.image_url,
    );
    
    let mut restaurants = db.restaurants.write().await;
    restaurants.insert(new_restaurant.id, new_restaurant.clone());
    
    Ok(Json(new_restaurant))
}

/// Get food items by restaurant (in real app, this would filter by restaurant)
pub async fn get_restaurant_menu(
    Path(_restaurant_id): Path<Uuid>,  // Fixed: added underscore to suppress warning
    State(db): State<Database>,
) -> Result<Json<Vec<FoodItem>>, (StatusCode, String)> {
    // For simplicity, return all food items
    // In a real app, you would filter by restaurant_id
    let food_items = db.food_items.read().await;
    let items: Vec<FoodItem> = food_items.values().cloned().collect();
    Ok(Json(items))
}

// ========== ORDER HANDLERS ==========

/// Get all orders
pub async fn get_orders(
    State(db): State<Database>,
) -> Result<Json<Vec<Order>>, (StatusCode, String)> {
    let orders = db.orders.read().await;
    let items: Vec<Order> = orders.values().cloned().collect();
    Ok(Json(items))
}

/// Get order by ID
pub async fn get_order(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<Json<Order>, (StatusCode, String)> {
    let orders = db.orders.read().await;
    match orders.get(&id) {
        Some(order) => Ok(Json(order.clone())),
        None => Err((StatusCode::NOT_FOUND, "Order not found".to_string())),
    }
}

/// Create new order
pub async fn create_order(
    State(db): State<Database>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<Order>, (StatusCode, String)> {
    // In real app, user_id would come from authentication
    // For this example, use the first user in the database
    let users = db.users.read().await;
    let user = users.values().next().ok_or_else(|| {
        (StatusCode::BAD_REQUEST, "No users found".to_string())
    })?;
    
    let restaurants = db.restaurants.read().await;
    let restaurant = restaurants.get(&payload.restaurant_id).ok_or_else(|| {
        (StatusCode::NOT_FOUND, "Restaurant not found".to_string())
    })?;
    
    // Get food items for validation and pricing
    let food_items = db.food_items.read().await;
    
    // Convert OrderItemRequest to OrderItem
    let mut order_items = Vec::new();
    let mut total_amount = 0.0;
    
    for item_request in payload.items {
        let food_item = food_items.get(&item_request.food_item_id).ok_or_else(|| {
            (StatusCode::NOT_FOUND, format!("Food item {:?} not found", item_request.food_item_id))
        })?;
        
        if !food_item.available {
            return Err((StatusCode::BAD_REQUEST, format!("Food item {} is not available", food_item.name)));
        }
        
        let order_item = OrderItem {
            food_item_id: item_request.food_item_id,
            quantity: item_request.quantity,
            price: food_item.price,
            name: food_item.name.clone(),
        };
        
        total_amount += order_item.price * order_item.quantity as f64;
        order_items.push(order_item);
    }
    
    // Check minimum order
    if total_amount < restaurant.minimum_order {
        return Err((StatusCode::BAD_REQUEST, 
            format!("Minimum order amount is ${:.2}", restaurant.minimum_order)));
    }
    
    // Add delivery fee
    total_amount += restaurant.delivery_fee;
    
    let new_order = Order {
        id: Uuid::new_v4(),
        user_id: user.id,
        restaurant_id: payload.restaurant_id,
        items: order_items,
        total_amount,
        delivery_address: payload.delivery_address,
        status: OrderStatus::Pending,
        notes: payload.notes,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let mut orders = db.orders.write().await;
    orders.insert(new_order.id, new_order.clone());
    
    Ok(Json(new_order))
}

/// Update order status
pub async fn update_order_status(
    Path(id): Path<Uuid>,
    State(db): State<Database>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<Order>, (StatusCode, String)> {
    let mut orders = db.orders.write().await;
    
    match orders.get_mut(&id) {
        Some(order) => {
            order.status = payload.status;
            order.updated_at = chrono::Utc::now();
            Ok(Json(order.clone()))
        }
        None => Err((StatusCode::NOT_FOUND, "Order not found".to_string())),
    }
}

/// Get user's orders
pub async fn get_user_orders(
    Path(user_id): Path<Uuid>,
    State(db): State<Database>,
) -> Result<Json<Vec<Order>>, (StatusCode, String)> {
    let orders = db.orders.read().await;
    let user_orders: Vec<Order> = orders.values()
        .filter(|order| order.user_id == user_id)
        .cloned()
        .collect();
    
    Ok(Json(user_orders))
}

// ========== USER HANDLERS ==========

/// Get all users
pub async fn get_users(
    State(db): State<Database>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = db.users.read().await;
    let items: Vec<User> = users.values().cloned().collect();
    Ok(Json(items))
}

/// Create new user
pub async fn create_user(
    State(db): State<Database>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let new_user = User::new(
        payload.name,
        payload.email,
        payload.phone,
        payload.address,
    );
    
    let mut users = db.users.write().await;
    users.insert(new_user.id, new_user.clone());
    
    Ok(Json(new_user))
}

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "Food Delivery API is running! 🚀"
}