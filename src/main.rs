mod database;

use database::Database;

fn main() {
    let mut db = Database::new();

    db.set("foo", "baz");

    // // populate db randomly
    // for i in 0..12 {
    //     db.set(i.to_string().as_str(), (i*10).to_string().as_str())
    // }

    println!("kvstore get: {} => {}", "foo", db.get("foo"));
}
