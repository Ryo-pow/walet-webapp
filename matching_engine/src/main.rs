use redis::Commands;
use serde::{Deserialize, Serialize};

// 注文データの構造体
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Order {
    id: u32,
    user_id: u32,  
    price: f64,
    amount: f64,   
    side: String,
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // 1. Redisへ接続 (localhost:6379)
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;

    // 2. 板（既存の売り注文リスト）を準備
    let mut orders = Vec::new();
    orders.push(Order { 
        id: 1, 
        user_id: 10,
        price: 50000.0, 
        amount: 1.0,
        side: String::from("sell") 
    });

    println!("🚀 Rust Matching Worker: 起動！ Redisを監視中...");

    // 3. 無限ループで「掲示板（Redis）」を監視し続ける
    loop {
        // "order_queue" という名前のキューからデータが来るまで待機 (BRPOP)
        // 0 は「データが来るまで無限に待つ」という設定です
        let (_, json_str): (String, String) = con.brpop("order_queue", 0.0)?;

        // 受信したJSON文字列をOrder構造体にデシリアライズ（変換）
        let new_order: Order = match serde_json::from_str(&json_str) {
            Ok(order) => order,
            Err(e) => {
                eprintln!("受信データの解析に失敗: {}", e);
                continue; // 失敗しても止まらずに次の注文を待つ
            }
        };

        println!("--- 注文を受信しました ---");
        println!("ID: {}, 価格: {}, 区分: {}", new_order.id, new_order.price, new_order.side);

        // 4. マッチング判定（以前のロジックをここに統合）
        for order in &orders {
            if order.side == "sell" && new_order.side == "buy" && new_order.price >= order.price {
                println!("⚡️ マッチング成立！");
                println!("取引成立: 売り(ID:{}) と 買い(ID:{})", order.id, new_order.id);
                println!("成約価格: {} / 数量: {}", order.price, new_order.amount);
            }
        }
        println!("✅ 処理完了。次の注文を待ちます...\n");
    }
}