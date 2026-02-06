use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Food Item Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub available: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FoodItem {
    pub fn new(name: String, description: String, price: f64, category: String, image_url: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            price,
            category,
            available: true,
            image_url,
            created_at: now,
            updated_at: now,
        }
    }
}

// Restaurant Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub address: String,
    pub phone: String,
    pub rating: f32,
    pub delivery_time: String, // e.g., "30-45 mins"
    pub delivery_fee: f64,
    pub minimum_order: f64,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Restaurant {
    pub fn new(
        name: String,
        description: String,
        address: String,
        phone: String,
        delivery_time: String,
        delivery_fee: f64,
        minimum_order: f64,
        image_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            address,
            phone,
            rating: 0.0,
            delivery_time,
            delivery_fee,
            minimum_order,
            image_url,
            created_at: now,
            updated_at: now,
        }
    }
}

// Order Item (food item with quantity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub food_item_id: Uuid,
    pub quantity: i32,
    pub price: f64,
    pub name: String,
}

// Order Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid, // In real app, this would come from authentication
    pub restaurant_id: Uuid,
    pub items: Vec<OrderItem>,
    pub total_amount: f64,
    pub delivery_address: String,
    pub status: OrderStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    #[allow(dead_code)]  // Add this line
    pub fn new(
        user_id: Uuid,
        restaurant_id: Uuid,
        items: Vec<OrderItem>,
        delivery_address: String,
        notes: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let total_amount = items.iter().map(|item| item.price * item.quantity as f64).sum();
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            restaurant_id,
            items,
            total_amount,
            delivery_address,
            status: OrderStatus::Pending,
            notes,
            created_at: now,
            updated_at: now,
        }
    }
}

// Order Status Enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Preparing,
    OutForDelivery,
    Delivered,
    Cancelled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Confirmed => write!(f, "confirmed"),
            OrderStatus::Preparing => write!(f, "preparing"),
            OrderStatus::OutForDelivery => write!(f, "out_for_delivery"),
            OrderStatus::Delivered => write!(f, "delivered"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

// User Model (simplified for this example)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String, phone: String, address: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            phone,
            address,
            created_at: now,
            updated_at: now,
        }
    }
}

// Request and Response Models
#[derive(Debug, Deserialize)]
pub struct CreateFoodItemRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRestaurantRequest {
    pub name: String,
    pub description: String,
    pub address: String,
    pub phone: String,
    pub delivery_time: String,
    pub delivery_fee: f64,
    pub minimum_order: f64,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub restaurant_id: Uuid,
    pub items: Vec<OrderItemRequest>,
    pub delivery_address: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemRequest {
    pub food_item_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}