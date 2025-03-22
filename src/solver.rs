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

        // 支払った人の残高を増やす
        update_balance(&mut balances, payer, amount);

        // 参加者の残高を減らす（端数は最後の参加者に割り当て）
        let share = amount / participants.len() as i32;
        let remainder = amount % participants.len() as i32;

        for (i, participant) in participants.iter().rev().enumerate() {
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

    // タプルからPaymentを作成するヘルパー関数
    fn payment_from_tuple(amount: i32, payer: &str, beneficiaries: Vec<&str>) -> Payment {
        Payment::new(
            Money::new(amount),
            Person::new(payer.to_string()),
            beneficiaries
                .into_iter()
                .map(|name| Person::new(name.to_string()))
                .collect(),
        )
    }

    // タプルからRepaymentを作成するヘルパー関数
    fn repayment_from_tuple(amount: i32, from: &str, to: &str) -> Repayment {
        Repayment::new(
            Money::new(amount),
            Person::new(from.to_string()),
            Person::new(to.to_string()),
        )
    }

    #[rstest]
    // ケース1: シンプルな支払い
    #[case(
        vec![(100, "A", vec!["B"])], 
        vec![(100, "B", "A")]
    )]
    // ケース2: 自分自身も受益者に含む場合
    #[case(
        vec![(100, "A", vec!["A", "B"])], 
        vec![(50, "B", "A")]
    )]
    // ケース3: 複数人への分割
    #[case(
        vec![(2000, "A", vec!["A", "B", "C", "D"])], 
        vec![
            (500, "B", "A"),
            (500, "C", "A"),
            (500, "D", "A"),
        ]
    )]
    // ケース4: 複数の支払い
    #[case(
        vec![
            (1200, "A", vec!["A", "B"]),
            (1200, "B", vec!["A", "B", "C", "D"]),
            (1200, "C", vec!["A", "B", "C", "D"]),
            (1200, "D", vec!["B", "C", "D"]),
        ],
        vec![
            (200, "B", "C"),
            (200, "B", "D"),
        ]
    )]
    // ケース5: より複雑なケース - 8人グループ、3つの支払い
    #[case(
        vec![
            (37009, "A", vec!["A", "B", "C", "D", "E", "F", "G", "H"]),
            (35300, "B", vec!["A", "B", "C", "D", "E", "F", "G", "H"]),
            (7249, "C", vec!["A", "B", "C", "D", "E", "F", "G", "H"]),
        ],
        vec![
            (2696, "C", "A"),
            (9944, "E", "A"),
            (9944, "G", "A"),
            (9944, "D", "B"),
            (9944, "F", "B"),
            (9944, "H", "B"),
            (4480, "B", "A"),
        ]
    )]
    // ケース6: 複数人グループ、多数の支払い
    #[case(
        vec![
            (200, "D", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (1500, "R", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (2767, "J", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (6100, "X", vec!["B", "D", "N", "P", "R", "X", "Y"]),
            (66857, "X", vec!["B", "D", "J", "N", "P", "S", "X", "Y"]),
            (690, "Y", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (6160, "Y", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (350, "N", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (330, "N", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
            (330, "N", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
        ],
        vec![
            (10554, "D", "X"),
            (7115, "J", "X"),
            (10754, "B", "X"),
            (10754, "P", "X"),
            (897, "R", "X"),
            (9882, "S", "X"),
            (8344, "N", "X"),
            (3904, "Y", "X"),
        ]
    )]
    fn test_solve(
        #[case] payment_tuples: Vec<(i32, &str, Vec<&str>)>,
        #[case] repayment_tuples: Vec<(i32, &str, &str)>,
    ) {
        let payments = payment_tuples
            .into_iter()
            .map(|(amount, payer, beneficiaries)| payment_from_tuple(amount, payer, beneficiaries))
            .collect();

        let expected: Vec<Repayment> = repayment_tuples
            .into_iter()
            .map(|(amount, from, to)| repayment_from_tuple(amount, from, to))
            .collect();

        let repayments = solve(payments);
        println!("repayments: {:?}", repayments);
        assert_eq!(repayments.len(), expected.len());
        for repayment in expected {
            assert!(repayments.contains(&repayment));
        }
    }
}
