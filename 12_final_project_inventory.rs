// LEVEL: Project
// TOPIC: Combines structs, enums, traits, errors, collections, iterators, tests
// RUN:   rustc --edition 2021 12_final_project_inventory.rs && ./12_final_project_inventory
// TEST:  rustc --edition 2021 --test 12_final_project_inventory.rs && ./12_final_project_inventory

use std::collections::HashMap;
use std::fmt;

// ---------- Data ----------
#[derive(Debug, Clone, PartialEq)]
struct Item {
    name: String,
    qty: u32,
    price: f64,
}

// ---------- Errors ----------
#[derive(Debug, PartialEq)]
enum InventoryError {
    NotFound(String),
    Insufficient { name: String, have: u32, want: u32 },
    InvalidPrice(f64),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InventoryError::NotFound(n) => write!(f, "item '{n}' not found"),
            InventoryError::Insufficient { name, have, want } => {
                write!(f, "not enough '{name}': have {have}, want {want}")
            }
            InventoryError::InvalidPrice(p) => write!(f, "invalid price {p}"),
        }
    }
}

impl std::error::Error for InventoryError {}

// ---------- Behavior shared by anything that can be valued ----------
trait Valuable {
    fn value(&self) -> f64;
}

impl Valuable for Item {
    fn value(&self) -> f64 {
        self.qty as f64 * self.price
    }
}

// ---------- Inventory ----------
#[derive(Default)]
struct Inventory {
    items: HashMap<String, Item>,
}

impl Inventory {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, name: &str, qty: u32, price: f64) -> Result<(), InventoryError> {
        if price <= 0.0 {
            return Err(InventoryError::InvalidPrice(price));
        }
        self.items
            .entry(name.to_string())
            .and_modify(|i| {
                i.qty += qty;
                i.price = price;
            })
            .or_insert(Item { name: name.to_string(), qty, price });
        Ok(())
    }

    fn remove(&mut self, name: &str, qty: u32) -> Result<(), InventoryError> {
        let item = self
            .items
            .get_mut(name)
            .ok_or_else(|| InventoryError::NotFound(name.to_string()))?;
        if item.qty < qty {
            return Err(InventoryError::Insufficient {
                name: name.to_string(),
                have: item.qty,
                want: qty,
            });
        }
        item.qty -= qty;
        Ok(())
    }

    fn total_value(&self) -> f64 {
        self.items.values().map(|i| i.value()).sum()
    }

    fn low_stock(&self, threshold: u32) -> Vec<&Item> {
        let mut v: Vec<&Item> = self.items.values().filter(|i| i.qty <= threshold).collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }

    fn report(&self) {
        let mut items: Vec<&Item> = self.items.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        println!("{:<10} {:>5} {:>8} {:>9}", "NAME", "QTY", "PRICE", "VALUE");
        for i in items {
            println!("{:<10} {:>5} {:>8.2} {:>9.2}", i.name, i.qty, i.price, i.value());
        }
        println!("TOTAL VALUE: {:.2}", self.total_value());
    }
}

// ---------- Main ----------
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inv = Inventory::new();
    inv.add("apple", 50, 0.5)?;
    inv.add("banana", 5, 0.25)?;
    inv.add("cherry", 200, 0.1)?;
    inv.add("apple", 10, 0.55)?; // restock updates qty and price

    inv.report();

    inv.remove("apple", 20)?;

    // Handle errors without crashing
    match inv.remove("banana", 10) {
        Err(e) => println!("expected error: {e}"),
        Ok(()) => println!("removed"),
    }
    if let Err(e) = inv.remove("durian", 1) {
        println!("expected error: {e}");
    }
    if let Err(e) = inv.add("ghost", 1, -3.0) {
        println!("expected error: {e}");
    }

    println!("\nAfter changes:");
    inv.report();

    let low: Vec<&str> = inv.low_stock(10).iter().map(|i| i.name.as_str()).collect();
    println!("low stock: {:?}", low);
    Ok(())
}

// ---------- Tests ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_total() {
        let mut inv = Inventory::new();
        inv.add("a", 2, 1.5).unwrap();
        inv.add("b", 1, 4.0).unwrap();
        assert!((inv.total_value() - 7.0).abs() < 1e-9);
    }

    #[test]
    fn remove_too_many_fails() {
        let mut inv = Inventory::new();
        inv.add("a", 2, 1.0).unwrap();
        assert_eq!(
            inv.remove("a", 5),
            Err(InventoryError::Insufficient { name: "a".into(), have: 2, want: 5 })
        );
    }

    #[test]
    fn missing_item() {
        let mut inv = Inventory::new();
        assert_eq!(inv.remove("x", 1), Err(InventoryError::NotFound("x".into())));
    }

    #[test]
    fn invalid_price() {
        let mut inv = Inventory::new();
        assert_eq!(inv.add("x", 1, 0.0), Err(InventoryError::InvalidPrice(0.0)));
    }
}

// TRY IT: Add a `save_to_file` / `load_from_file` using std::fs (or the
// `serde` + `serde_json` crates), a CLI using `clap`, and a `search` method.
