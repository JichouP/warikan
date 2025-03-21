use crate::entity::{money::Money, payment::Payment, person::Person, repayment::Repayment};
use std::collections::HashMap;

fn update_balance(balances: &mut HashMap<String, i32>, person: String, amount: i32) {
    *balances.entry(person).or_insert(0) += amount;
}

pub fn solve(payments: Vec<Payment>) -> Vec<Repayment> {
    // 支払いの数から初期容量を推定
    let estimated_capacity = payments.len() * 2;
    let mut balances: HashMap<String, i32> = HashMap::with_capacity(estimated_capacity);

    for payment in payments {
        let payer = payment.from().as_inner().clone();
        let amount = payment.money().as_inner();
        let participants = payment.to();

        // 参加者が空の場合はスキップ
        if participants.is_empty() {
            continue;
        }

        let share = amount / participants.len() as i32;
        let remainder = amount % participants.len() as i32;

        // 支払った人の残高を増やす
        update_balance(&mut balances, payer, amount);

        // 参加者の残高を減らす（端数は最初の参加者に割り当て）
        for (i, participant) in participants.iter().enumerate() {
            let name = participant.as_inner().clone();
            let share_amount = if i == 0 { share + remainder } else { share };
            update_balance(&mut balances, name, -share_amount);
        }
    }

    // プラス残高とマイナス残高の人々を分ける
    let mut creditors: Vec<(String, i32)> = Vec::new();
    let mut debtors: Vec<(String, i32)> = Vec::new();

    for (person, balance) in balances {
        match balance {
            b if b > 0 => creditors.push((person, balance)),
            b if b < 0 => debtors.push((person, balance)),
            _ => {}
        }
    }

    // 残高の絶対値が大きい順にソート
    creditors.sort_by(|a, b| b.1.cmp(&a.1));
    debtors.sort_by(|a, b| a.1.cmp(&b.1));

    // 返済リストを作成
    let mut repayments = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < debtors.len() && j < creditors.len() {
        let debtor = &debtors[i];
        let creditor = &creditors[j];

        // 返済額を計算（小さい方の絶対値）
        let repayment_amount = std::cmp::min(creditor.1, -debtor.1);

        // 返済を作成
        repayments.push(Repayment::new(
            Money::new(repayment_amount),
            Person::new(debtor.0.clone()),
            Person::new(creditor.0.clone()),
        ));

        // 残高を更新
        let new_debtor_balance = debtor.1 + repayment_amount;
        let new_creditor_balance = creditor.1 - repayment_amount;

        // 残高が0になった人は次へ
        if new_debtor_balance == 0 {
            i += 1;
        }

        if new_creditor_balance == 0 {
            j += 1;
        }
    }

    repayments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{money::Money, person::Person};
    use rstest::rstest;

    #[rstest]
    #[case(
        vec![Payment::new(
            Money::new(100),
            Person::new("A".to_string()),
            vec![Person::new("B".to_string())],
        )],
        vec![Repayment::new(
            Money::new(100),
            Person::new("B".to_string()),
            Person::new("A".to_string()),
        )]
    )]
    #[case(
        vec![Payment::new(
            Money::new(100),
            Person::new("A".to_string()),
            vec![
                Person::new("A".to_string()),
                Person::new("B".to_string()),
            ],
        )],
        vec![Repayment::new(
            Money::new(50),
            Person::new("B".to_string()),
            Person::new("A".to_string()),
        )]
    )]
    #[case(
        vec![Payment::new(
            Money::new(2000),
            Person::new("A".to_string()),
            vec![
                Person::new("A".to_string()),
                Person::new("B".to_string()),
                Person::new("C".to_string()),
                Person::new("D".to_string()),
            ],
        )],
        vec![
            Repayment::new(
                Money::new(500),
                Person::new("B".to_string()),
                Person::new("A".to_string()),
            ),
            Repayment::new(
                Money::new(500),
                Person::new("C".to_string()),
                Person::new("A".to_string()),
            ),
            Repayment::new(
                Money::new(500),
                Person::new("D".to_string()),
                Person::new("A".to_string()),
            ),
        ]
    )]
    #[case(
        vec![Payment::new(
            Money::new(1200),
            Person::new("A".to_string()),
            vec![
                Person::new("A".to_string()),
                Person::new("B".to_string()),
            ],
        ),
        Payment::new(
            Money::new(1200),
            Person::new("B".to_string()),
            vec![
                Person::new("A".to_string()),
                Person::new("B".to_string()),
                Person::new("C".to_string()),
                Person::new("D".to_string()),
            ],
        ),
        Payment::new(
            Money::new(1200),
            Person::new("C".to_string()),
            vec![
                Person::new("A".to_string()),
                Person::new("B".to_string()),
                Person::new("C".to_string()),
                Person::new("D".to_string()),
            ],
        ),
        Payment::new(
            Money::new(1200),
            Person::new("D".to_string()),
            vec![
                Person::new("B".to_string()),
                Person::new("C".to_string()),
                Person::new("D".to_string()),
            ],
        ),
    ],
        vec![
            Repayment::new(
                Money::new(200),
                Person::new("B".to_string()),
                Person::new("C".to_string()),
            ),
            Repayment::new(
                Money::new(200),
                Person::new("B".to_string()),
                Person::new("D".to_string()),
            ),
        ]
    )]
    fn test_solve(#[case] payments: Vec<Payment>, #[case] expected: Vec<Repayment>) {
        let repayments = solve(payments);
        assert_eq!(repayments.len(), expected.len());
        for repayment in expected {
            assert!(repayments.contains(&repayment));
        }
    }
}
