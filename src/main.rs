mod utils;

fn main() {
    let users = vec!["Alice", "Bob", "Charlie"];
    let orders = vec![Order { user_id: "Alice", item: "Book" }, Order { user_id: "Bob", item: "Pen" }];
    let results = utils::get_orders_by_user(&users, &orders);
    for result in results {
        println!("{}", result);
    }
}

struct Order {
    user_id: &'static str,
    item: &'static str,
}