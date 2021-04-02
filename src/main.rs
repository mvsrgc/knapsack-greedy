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

struct KnapSack {
    items: Vec<Item>,
    max_weight: f64,
    current_weight: f64,
    max_value: f64,
}

impl KnapSack {
    pub fn new(items: Vec<Item>, max_weight: f64) -> Self {
        KnapSack {
            items,
            max_weight,
            current_weight: 0.0,
            max_value: 0.0,
        }
    }

    pub fn max_value(&mut self) -> f64 {
        if self.items.len() == 0 || self.max_weight == 0.0 {
            return 0.0
        }

        // Sort the list so that we add the items with the highest ratio first.
        self.items.sort_by(|a, b| b.ratio().partial_cmp(&a.ratio()).unwrap());

        // Choose items
        let iter = self.items.iter();

        for item in iter {
            if self.current_weight + item.weight <= self.max_weight {
                self.current_weight += item.weight;
                self.max_value += item.value;
            } else {
                // If this item is too large, we can pick a fractional part of it.
                let remaining_weight = self.max_weight - self.current_weight;
                self.max_value += item.value * (remaining_weight / item.weight);

                // Bag is now full
                self.current_weight = self.max_weight;
            }
        }

        self.max_value
    }
}

fn main() {
    let items: Vec<Item> = vec![Item::new(60.0, 10.0), Item::new(100.0, 20.0), Item::new(120.0, 30.0)];
    let mut knapsack: KnapSack = KnapSack::new(items, 50.0);
    println!("{}", knapsack.max_value());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stuff() {
        let mut items: Vec<Item> = vec![Item::new(60.0, 10.0)];
        let mut knapsack: KnapSack = KnapSack::new(items, 60.0);
        assert_eq!(knapsack.max_value(), 60.0);

        items = vec![Item::new(60.0, 10.0)];
        knapsack = KnapSack::new(items, 5.0);
        assert_eq!(knapsack.max_value(), 30.0);

        items = vec![Item::new(60.0, 10.0), Item::new(10.0, 5.0)];
        knapsack = KnapSack::new(items, 15.0);
        assert_eq!(knapsack.max_value(), 70.0);

        items = vec![Item::new(60.0, 10.0), Item::new(10.0, 5.0)];
        knapsack = KnapSack::new(items, 20.0);
        assert_eq!(knapsack.max_value(), 70.0);

        items = vec![];
        knapsack = KnapSack::new(items, 20.0);
        assert_eq!(knapsack.max_value(), 0.0);

        items = vec![Item::new(60.0, 10.0), Item::new(100.0, 20.0), Item::new(120.0, 30.0)];
        knapsack = KnapSack::new(items, 50.0);
        assert_eq!(knapsack.max_value(), 240.0);

        items = vec![Item::new(60.0, 10.0), Item::new(100.0, 20.0), Item::new(120.0, 30.0)];
        knapsack = KnapSack::new(items, 0.0);
        assert_eq!(knapsack.max_value(), 0.0);
    }
}