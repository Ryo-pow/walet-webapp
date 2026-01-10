struct Order {
    id: u32,
    user_id: u32,  // 追加
    price: f64,
    amount: f64,   // 追加
    side: String,
}

fn main() {
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
        if order.side == "sell" && new_buy_order.price >= order.price {
            println!("⚡️ マッチング成立！");
            println!("取引成立: 売り注文(ID:{}) と 買い注文(ID:{})", order.id, new_buy_order.id);
            println!("価格: {} / 数量: {}", order.price, new_buy_order.amount);
        }
    }
}