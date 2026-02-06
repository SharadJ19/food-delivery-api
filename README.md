# 🍕 Food Delivery REST API 🚀

<div align="center">

**A deliciously fast food delivery API built with Rust & Axum**  
*Serving happiness one byte at a time!*

</div>

## ✨ Features

| Feature | Emoji | Description |
|---------|-------|-------------|
| **Restaurant Management** | 🏪 | Create, read, update restaurants |
| **Food Menu Management** | 🍔 | Manage delicious food items |
| **Order Processing** | 📦 | Handle orders from cart to delivery |
| **User Management** | 👥 | User profiles and accounts |
| **Order Tracking** | 📍 | Real-time order status updates |
| **In-memory Database** | 💾 | Lightning-fast data storage |

## 🚀 Quick Start

### 1️⃣ **Prerequisites**
```bash
🦀 Rust installed
📦 Cargo package manager
🌐 curl or Postman for testing
```

### 2️⃣ **Installation & Run**
```bash
# Clone & Navigate
📂 git clone https://github.com/sharadj19/food-delivery-api
📂 cd food-delivery-api

# Build & Run
⚡ cargo build
🚀 cargo run

# 🎉 Server running at: http://localhost:3000
```

### 3️⃣ **Verify Installation**
```bash
🔍 curl http://localhost:3000/health
📝 Response: "Food Delivery API is running! 🚀"
```

## 📡 API Endpoints

### 🩺 Health Check
- `GET /health` 🟢 → Check API heartbeat

### 🍽️ Food Items
| Method | Endpoint | Action | Emoji |
|--------|----------|--------|-------|
| `GET` | `/food-items` | 📋 List all items | 🍕 |
| `POST` | `/food-items` | ➕ Create new item | 🍔 |
| `GET` | `/food-items/:id` | 🔍 Get by ID | 🍟 |
| `DELETE` | `/food-items/:id` | 🗑️ Delete item | 🧹 |

### 🏪 Restaurants
| Method | Endpoint | Action | Emoji |
|--------|----------|--------|-------|
| `GET` | `/restaurants` | 📋 List all | 🏬 |
| `POST` | `/restaurants` | ➕ Create new | 🆕 |
| `GET` | `/restaurants/:id` | 🔍 Get details | 🔎 |
| `GET` | `/restaurants/:id/menu` | 📜 Get menu | 📋 |

### 📦 Orders
| Method | Endpoint | Action | Emoji |
|--------|----------|--------|-------|
| `GET` | `/orders` | 📋 List all orders | 📄 |
| `POST` | `/orders` | ➕ Place order | 🛒 |
| `GET` | `/orders/:id` | 🔍 Track order | 🗺️ |
| `PUT` | `/orders/:id/status` | 🔄 Update status | ⏫ |
| `GET` | `/users/:user_id/orders` | 👤 User orders | 👤 |

### 👥 Users
| Method | Endpoint | Action | Emoji |
|--------|----------|--------|-------|
| `GET` | `/users` | 📋 List users | 👥 |
| `POST` | `/users` | ➕ Register user | ✍️ |


## 🧪 API Testing Examples

### 🏪 **Create a Restaurant**
```bash
curl -X POST http://localhost:3000/restaurants \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Pizza Paradise 🍕",
    "description": "Heavenly pizzas! 😇",
    "address": "123 Pizza Street",
    "phone": "🍕-PIZZA-123",
    "delivery_time": "⚡ 20-30 mins",
    "delivery_fee": 2.99,
    "minimum_order": 15.0,
    "image_url": "https://example.com/pizza-paradise.jpg"
  }'
```

### 🍔 **Add Food Item**
```bash
curl -X POST http://localhost:3000/food-items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Double Cheeseburger 🍔",
    "description": "Twice the cheese, double the joy! 😋",
    "price": 11.99,
    "category": "Burgers 🍔",
    "image_url": "https://example.com/double-cheese.jpg"
  }'
```

### 🛒 **Place an Order**
```bash
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "restaurant_id": "RESTAURANT_UUID_HERE",
    "items": [
      {
        "food_item_id": "FOOD_UUID_HERE",
        "quantity": 2
      }
    ],
    "delivery_address": "🏠 456 Home Street",
    "notes": "🚫 No onions please!"
  }'
```

## 🏗️ Project Structure

```plaintext
food-delivery-api/
├── 📁 src/
│   ├── 🎯 main.rs         # 🚀 Launchpad
│   ├── 🏗️ models.rs       # 📐 Blueprints
│   ├── 🎮 handlers.rs     # 🎯 Controllers
│   ├── 🗃️ db.rs           # 💾 Data Store
│   └── 🛣️ routes.rs       # 🗺️ Roadmap
├── 📄 Cargo.toml          # 📦 Toolbox
├── 🔧 .env                # 🔐 Secrets
└── 📖 README.md           # 📚 Storybook
```

## ⚙️ Tech Stack

| Component | Technology | Emoji |
|-----------|------------|-------|
| **Framework** | Axum (Rust) | 🦀 |
| **Serialization** | Serde | 📦 |
| **UUID Generation** | uuid | 🆔 |
| **Time Handling** | Chrono | ⏰ |
| **CORS** | tower-http | 🌐 |
| **Environment** | dotenv | 🔧 |


## ⚠️ Important Notes

| Note | Emoji | Details |
|------|-------|---------|
| **Database** | 💾 | In-memory (data lost on restart) |
| **Authentication** | 🔓 | Simplified for demo |
| **Error Handling** | ⚠️ | Basic implementation |
| **Production Ready** | 🚧 | Learning project |


## 🚀 Next Steps for Production

1. **Database** 🗄️ → PostgreSQL with SQLx
2. **Authentication** 🔐 → JWT with proper auth
3. **Validation** ✅ → Request data validation
4. **Error Handling** 🚨 → Comprehensive error responses
5. **Logging** 📝 → Structured logging
6. **Testing** 🧪 → Unit & integration tests
7. **Rate Limiting** ⏱️ → Prevent abuse
8. **Containerization** 📦 → Docker setup
9. **CI/CD** 🔄 → GitHub Actions
10. **Monitoring** 📊 → Metrics & alerts

## 🤝 Contributing

```bash
1. 🍴 Fork the repository
2. 🌿 Create a feature branch
3. ✨ Add your magic
4. 🔀 Create a Pull Request
5. 🎉 Celebrate contribution!
```

## 🎯 Quick Commands Cheat Sheet

```bash
# 🏗️ Build
cargo build

# 🚀 Run
cargo run

# 🧪 Test
cargo test

# 📦 Check
cargo check

# 🧹 Clean
cargo clean

# 📊 Dependencies
cargo tree
```

<div align="center">

## 🎉 Happy Coding! 🚀

**Built with ❤️ and 🦀 Rust**

⭐ **Star this repo if you found it helpful!** ⭐

**"Good code is like a good pizza - it should be simple, delicious, and leave you wanting more!"** 🍕

</div>