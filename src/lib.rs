pub mod orderbook {

    #[derive(Debug,PartialEq)]
    pub enum EntryType {
        BID,
        ASK,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Entry {
        pub id: i32,
        pub side: EntryType,
        pub price: i64,
        pub quantity: i64,
    }

    impl Eq on Entry  {
        fn eq(&self, other: &Entry) -> bool {
             other.id

        }
    }

    #[derive(Debug, PartialEq)]
    pub enum OrderType {
        MARKET,
        LIMIT,
    }

    #[derive(Debug, PartialEq)]
    pub enum OrderSide {
        BUY,
        SELL,
    }

    #[derive(Debug)]
    pub struct Order {
        pub id: i32,
        pub side: OrderSide,
        pub limit: Option<i32>,
        pub quantity: i64,
        pub order_type: OrderType,
    }

    #[derive(Clone,Debug,Copy,PartialEq)]
    pub struct ExecutedOrder {
        pub order_id: i32,
        pub entry_id: i32,
        pub quantity: i64,
        pub execution_price: i64,
    }

    pub struct OrderBook {
        //for the sake of the demo the entries data structure is Vector
        pub entries: Vec<Entry>,
        pub executed_orders: Vec<ExecutedOrder>,
    }

    impl OrderBook {
        pub fn execute(&mut self, o: Order) {
            if o.side == OrderSide::BUY {
                self.execute_buy(o);
            }
        }

        fn execute_buy(&mut self, o: Order) {
            let mut asks: Vec<&Entry> = self.entries.iter().filter(
                |e| e.side == EntryType::ASK && e.quantity >= o.quantity
            ).collect();

            asks.sort_by(|a, b| a.price.cmp(&b.price));
            let mut quantity = o.quantity;
            for (i,entry) in asks.iter().enumerate() {
                if o.order_type == OrderType::MARKET {
                    self.executed_orders.push(
                        ExecutedOrder {
                            order_id: o.id,
                            entry_id: entry.id,
                            quantity: o.quantity,
                            execution_price: entry.price,
                        });
                    quantity -= o.quantity;
                    if quantity == 0 {
                        self.entries.remove(i);
                        break
                    }
                }
            }
        }

        pub fn executed_orders(&self) -> Vec<ExecutedOrder> {
            return self.executed_orders.clone();
        }
    }
}


#[cfg(test)]
mod tests {
    use orderbook::*;

    use crate::orderbook;

    #[test]
    fn assert_works() {
        let result = 2 * 2;
        assert_eq!(result, 2 + 2);
    }

    #[test]
    fn assert_entries() {
        let o = Order { id: 1, side: OrderSide::BUY, quantity: 100, order_type: OrderType::MARKET, limit: None };
        let mut entries = Vec::new();
        entries.push(Entry { id: 1, side: EntryType::ASK, price: 10010, quantity: 100 });
        entries.push(Entry { id: 2, side: EntryType::ASK, price: 10010, quantity: 100 });
        entries.push(Entry { id: 3, side: EntryType::ASK, price: 10005, quantity: 500 });
        entries.push(Entry { id: 4, side: EntryType::ASK, price: 10000, quantity: 1000 });
        entries.push(Entry { id: 5, side: EntryType::BID, price: 9995, quantity: 100 });
        entries.push(Entry { id: 6, side: EntryType::BID, price: 9990, quantity: 50 });
        entries.push(Entry { id: 7, side: EntryType::BID, price: 9985, quantity: 50 });
        let mut book = OrderBook { entries: entries, executed_orders: Vec::new() };
        book.execute(o);

        let executed = book.executed_orders();

        assert_eq!(executed, vec![ExecutedOrder{order_id: 1, entry_id: 4, quantity: 100, execution_price: 10000 }]);
        assert_eq!(false, book.entries.contains(&Entry { id: 1, side: EntryType::ASK, price: 10010, quantity: 100 }));

    }
}