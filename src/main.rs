#[derive(Debug)]
struct Item {
    value: f64,
    weight: f64,

}

impl Item {
    pub fn new(value: f64, weight: f64) -> Self {
        Item {
            value: value,
            weight: weight,
        }
    }

    pub fn ratio(&self) -> f64 {
        self.value as f64 / self.weight as f64
    }
}

fn main() {
    let max_weight: f64 = 50.0;
    let mut current_weight: f64 = 0.0;
    let mut current_value: f64 = 0.0;

    let mut items: Vec<Item> = vec![Item::new(60.0, 10.0), Item::new(100.0, 20.0), Item::new(120.0, 30.0)];

    // Sort the list so that we add the items with the highest ratio first.
    items.sort_by(|a, b| b.ratio().partial_cmp(&a.ratio()).unwrap());

    // Choose items
    for x in 0..items.len() {
        if current_weight + items[x].weight <= max_weight {
            current_weight += items[x].weight;
            current_value += items[x].value;
        } else {
            // If this item is too large, we can pick a fractional part of it.
            let remaining_weight = max_weight - current_weight;
            current_value += items[x].value * (remaining_weight / items[x].weight);

            // Bag is now full
            current_weight = max_weight;
        }
    }

    println!("{:?}", current_value);
}
