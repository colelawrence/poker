pub(crate) mod card;
pub(crate) mod cashier;
pub(crate) mod deck;

pub mod poker;

#[test]
fn test_chips() {
    let mut cashier = cashier::Cashier::new(10000);
    let mut chips = cashier.buy_chips(10).unwrap();
    assert_eq!(chips.count(), 10);

    let five = chips.take(5).unwrap();
    assert_eq!(five.count(), 5);
    assert_eq!(chips.count(), 5);
    chips.add(five);
    cashier.exchange_chips(chips);
}
