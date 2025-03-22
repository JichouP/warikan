use crate::entity::{money::Money, payment::Payment, person::Person, repayment::Repayment};
use std::collections::HashMap;

pub fn solve(payments: Vec<Payment>) -> Vec<Repayment> {
    // 各人の残高を計算
    let mut balances: HashMap<String, i32> = HashMap::new();

    for payment in payments {
        let payer = payment.from().as_inner().clone();
        let amount = payment.money().as_inner();
        let participants = payment.to();

        // 参加者が空の場合はスキップ
        if participants.is_empty() {
            continue;
        }

        // 支払った人は支払い金額を追加（債権）
        *balances.entry(payer.clone()).or_insert(0) += amount;

        // 各参加者の負担分を計算
        let share_per_person = amount / participants.len() as i32;
        let remainder = amount % participants.len() as i32;

        // 参加者ごとに負担額を差し引く（債務）
        for (i, participant) in participants.iter().enumerate() {
            let name = participant.as_inner().clone();

            // 最後の参加者に余りを加算
            let share = if i == participants.len() - 1 {
                share_per_person + remainder
            } else {
                share_per_person
            };

            // 自分自身が支払者かつ参加者の場合は相殺
            if name == payer {
                *balances.get_mut(&name).unwrap() -= share;
            } else {
                *balances.entry(name).or_insert(0) -= share;
            }
        }
    }

    // 債権者と債務者に分ける
    let mut creditors: Vec<(String, i32)> = Vec::new();
    let mut debtors: Vec<(String, i32)> = Vec::new();

    for (person, balance) in balances {
        match balance.cmp(&0) {
            std::cmp::Ordering::Greater => creditors.push((person, balance)),
            std::cmp::Ordering::Less => debtors.push((person, -balance)),
            std::cmp::Ordering::Equal => {}
        }
    }

    // 債権額の大きい順にソート、金額が同じ場合は名前で安定的にソート
    creditors.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    debtors.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // 返済計画を生成
    let mut repayments = Vec::new();

    // 再帰的に清算計算を行う
    calculate_repayments(&mut creditors, &mut debtors, &mut repayments);

    // 返済リストを from のアルファベット順 → to のアルファベット順でソート
    repayments.sort_by(|a, b| {
        a.from()
            .as_inner()
            .cmp(b.from().as_inner())
            .then_with(|| a.to().as_inner().cmp(b.to().as_inner()))
    });

    repayments
}

fn calculate_repayments(
    creditors: &mut Vec<(String, i32)>,
    debtors: &mut Vec<(String, i32)>,
    repayments: &mut Vec<Repayment>,
) {
    // 債権者または債務者がいなくなったら終了
    if creditors.is_empty() || debtors.is_empty() {
        return;
    }

    // 最大債権者と最大債務者を取得
    let creditor = &creditors[0];
    let debtor = &debtors[0];

    // 清算金額 = min(債権額, 債務額)
    let amount = std::cmp::min(creditor.1, debtor.1);

    // 金額が0なら処理終了
    if amount == 0 {
        return;
    }

    // 返済を記録
    repayments.push(Repayment::new(
        Money::new(amount),
        Person::new(debtor.0.clone()),
        Person::new(creditor.0.clone()),
    ));

    // 債権者の残高を減らす
    creditors[0].1 -= amount;

    // 債務者の残高を減らす
    debtors[0].1 -= amount;

    // 残高が0になった人を削除
    if creditors[0].1 == 0 {
        creditors.remove(0);
    }

    if debtors[0].1 == 0 {
        debtors.remove(0);
    }

    // 次の清算を計算
    calculate_repayments(creditors, debtors, repayments);
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
            (2695, "C", "B"),
            (9944, "D", "A"),
            (7171, "E", "A"),
            (2773, "E", "B"),
            (9944, "F", "B"),
            (9944, "G", "B"),
            (9950, "H", "A"),
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
            (1400, "N", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
        ],
        vec![
            (10748, "B", "X"),
            (10548, "D", "X"),
            (7110, "J", "X"),
            (8338, "N", "X"),
            (10748, "P", "X"),
            (891, "R", "X"),
            (9877, "S", "X"),
            (3949, "Y", "X"),
        ]
    )]
    // ケース7: 金額が0の支払いを含むケース
    #[case(
        vec![
            (0, "A", vec!["B"]),
            (100, "A", vec!["B"]),
        ],
        vec![
            (100, "B", "A"),
        ]
    )]
    // ケース8: 参加者が空のケース
    #[case(
        vec![
            (100, "A", vec![]),
            (200, "B", vec!["A", "B"]),
        ],
        vec![
            (100, "A", "B"),
        ]
    )]
    // ケース9: 全員の収支が0になるケース
    #[case(
        vec![
            (100, "A", vec!["B"]),
            (100, "B", vec!["A"]),
        ],
        vec![]
    )]
    // ケース10: 同額の債権・債務が複数存在するケース（ソートの安定性）
    #[case(
        vec![
            (100, "A", vec!["C", "D"]),
            (100, "B", vec!["C", "D"]),
        ],
        vec![
            (100, "C", "A"),
            (100, "D", "B"),
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
        assert_eq!(repayments, expected);
        // for repayment in expected {
        //     assert!(repayments.contains(&repayment));
        // }
    }
}
