use crate::engine::constants::STRING_SIZE;
use crate::state::{MerchantAccount, OrderAccount, SubscriptionAccount};

/// Given the expected amount, calculate the fee and take home amount
/// Currently fee is 0.3% with a minimum fee of 1 lamport
/// If the amount is less than 100 lamports the fee is 0
pub fn get_amounts(amount: u64, fee_percentage: u128) -> (u64, u64) {
    let mut fee_amount: u64 = 0;
    let mut take_home_amount: u64 = amount;

    if amount >= 100 {
        let possible_fee_amount: u128 = (amount as u128 * fee_percentage) / 1000;
        fee_amount = 1;
        if possible_fee_amount > 0 {
            fee_amount = possible_fee_amount as u64;
        }
        take_home_amount = amount - fee_amount;
    }

    (take_home_amount, fee_amount)
}

pub fn get_account_size(min_len: usize, strings: &Vec<&String>) -> usize {
    let mut size = min_len;
    for item in strings {
        size = size + item.chars().count() + STRING_SIZE;
    }

    size
}

/// get order account size
pub fn get_order_account_size(order_id: &String, secret: &String, data: &String) -> usize {
    get_account_size(OrderAccount::MIN_LEN, &vec![order_id, secret, data])
}

/// get merchant account size
pub fn get_merchant_account_size(data: &String) -> usize {
    get_account_size(MerchantAccount::MIN_LEN, &vec![data])
}

/// get subscription account size
pub fn get_subscription_account_size(name: &String, data: &String) -> usize {
    get_account_size(SubscriptionAccount::MIN_LEN, &vec![name, data])
}

#[cfg(test)]
mod test {
    use {super::*, solana_program_test::*};

    #[tokio::test]
    async fn test_get_amounts() {
        assert_eq!((997000000, 3000000), get_amounts(1000000000, 3));
        assert_eq!((1994000, 6000), get_amounts(2000000, 3));
        assert_eq!((1994, 6), get_amounts(2000, 3));
        assert_eq!((100, 1), get_amounts(101, 3));
        assert_eq!((99, 1), get_amounts(100, 3));
        assert_eq!((99, 0), get_amounts(99, 3));
        assert_eq!((80, 0), get_amounts(80, 3));
        assert_eq!((0, 0), get_amounts(0, 3));
        assert_eq!((990, 10), get_amounts(1000, 10));
        assert_eq!((996, 4), get_amounts(1000, 4));
    }

    #[tokio::test]
    async fn test_get_order_account_size() {
        assert_eq!(
            198,
            get_order_account_size(
                &String::from("123456"),
                &String::from("password"),
                &String::from(r#"{"a": "b"}"#)
            )
        );
        assert_eq!(
            190,
            get_order_account_size(
                &String::from("test-6"),
                &String::from(""),
                &String::from(r#"{"a": "b"}"#)
            )
        );
        assert_eq!(423, get_order_account_size(&String::from("WSUDUBDG2"), &String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type"), &String::from(r#"{"a": "b"}"#)));
    }

    #[tokio::test]
    async fn test_get_merchant_account_size() {
        assert_eq!(79, get_merchant_account_size(&String::from("{}")));
        assert_eq!(
            168,
            get_merchant_account_size(&String::from(
                r#"{"code":200,"success":true,"payload":{"features":["awesome","easyAPI","lowLearningCurve"]}}"#
            ))
        );
    }

    #[tokio::test]
    async fn test_get_subscription_account_size() {
        assert_eq!(
            100,
            get_subscription_account_size(&String::from("a"), &String::from("b"))
        );
        assert_eq!(
            132,
            get_subscription_account_size(
                &String::from("Annual"),
                &String::from(r#"{"foo": "bar", "price": 200}"#)
            )
        );
    }
}
