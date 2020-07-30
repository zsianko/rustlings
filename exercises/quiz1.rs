// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

const PRICE: i32 = 2;
const WHOLESALE_PRICE: i32 = 1;
const WHOLESALE_AMOUNT: i32 = 40;

fn calculate_apple_price(amount: i32) -> i32 {
    if amount > WHOLESALE_AMOUNT {
        amount * WHOLESALE_PRICE
    } else {
        amount * PRICE
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
