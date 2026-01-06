// 注文を表す構造体
struct Order {
    id: u32,
    user_id: u32,
    price: f64,
    amount: f64,
    side: String, // "buy"（買い） か "sell"（売り）
}

fn main() {
    // 試しに一つの「買い注文」を作ってみる
    let buy_order = Order {
        id: 1,
        user_id: 1,
        price: 50000.0,
        amount: 0.5,
        side: String::from("buy"),
    };

    println!("--- Ultra-Trade Engine: Matching Unit ---");
    println!("注文を受け付けました: ID={}, ユーザー={}, 価格={}, 数量={}, 区分={}", 
        buy_order.id, buy_order.user_id, buy_order.price, buy_order.amount, buy_order.side);
}