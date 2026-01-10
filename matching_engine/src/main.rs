use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Order {
    id: u32,
    user_id: u32,  
    price: f64,
    amount: f64,   
    side: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/match", post(handle_match));
    
    println!(" Rust Matching Engine: ポート3001で待機中...");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

 async fn handle_match(Json(new_order): Json<Order>) -> Json<String> {
    println!(">>>>>> Rustがデータを受け取りました！ ID: {}", new_order.id);
    println!("注文を受信: ID={}, 価格={}", new_order.id, new_order.price);

    // 1. 注文リスト（板：オーダーブック）を作成
    let mut orders = Vec::new();

     // 2. 既存の「売り注文」を追加
    orders.push(Order { 
        id: 1, 
        user_id: 10,
        price: 50000.0, 
        amount: 1.0,
        side: String::from("sell") 
    });

    // 3. 新しく「買い注文」が来たとする
    let new_buy_order = Order { 
        id: 2, 
        user_id: 11,
        price: 50100.0, 
        amount: 0.5,
        side: String::from("buy") 
    };

    println!("--- Ultra-Trade Engine: Matching Unit ---");
    println!("新しく買い注文が届きました: 価格 {}", new_buy_order.price);

    // 4. マッチング判定（ordersの中身を一つずつチェック）
    for order in &orders {
        // 条件：相手が「売り」で、自分の「買い価格」の方が高いか同じなら成立
        if order.side == "sell" && new_order.side == "buy" && new_order.price >= order.price {
            println!("⚡️ マッチング成立！");
            println!("取引成立: 売り注文(ID:{}) と 買い注文(ID:{})", order.id, new_order.id);
            println!("価格: {} / 数量: {}", order.price, new_order.amount);
            }
        }
        Json(format!("Order {} received and processed", new_order.id))
    }
