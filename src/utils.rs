use std::collections::HashMap;

pub fn get_orders_by_user(users: &Vec<&str>, orders: &Vec<Order>) -> Vec<String> {
    let mut orders_by_user: HashMap<&str, Vec<&Order>> = HashMap::new();
    for order in orders {
        orders_by_user.entry(order.user_id).or_insert(Vec::new()).push(order);
    }
    let mut results = Vec::new();
    for user in users {
        if let Some(user_orders) = orders_by_user.get(user) {
            for order in user_orders {
                results.push(format!("User: {}, Item: {}", order.user_id, order.item));
            }
        }
    }
    results
}

pub struct Order {
    pub user_id: &'static str,
    pub item: &'static str,
}