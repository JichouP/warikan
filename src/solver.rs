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
        if balance > 0 {
            // プラス残高は債権者
            creditors.push((person, balance));
        } else if balance < 0 {
            // マイナス残高は債務者
            debtors.push((person, -balance)); // 負の値を正にして保存
        }
    }

    // 債権額の大きい順にソート
    creditors.sort_by(|a, b| b.1.cmp(&a.1));

    // 債務額の大きい順にソート
    debtors.sort_by(|a, b| b.1.cmp(&a.1));

    // 返済計画を生成
    let mut repayments = Vec::new();

    // 再帰的に清算計算を行う
    calculate_repayments(&mut creditors, &mut debtors, &mut repayments);

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

    // 残高を更新
    let mut updated_creditors = creditors.clone();
    let mut updated_debtors = debtors.clone();

    // 債権者の残高を減らす
    updated_creditors[0].1 -= amount;

    // 債務者の残高を減らす
    updated_debtors[0].1 -= amount;

    // 残高が0になった人を削除
    if updated_creditors[0].1 == 0 {
        updated_creditors.remove(0);
    }

    if updated_debtors[0].1 == 0 {
        updated_debtors.remove(0);
    }

    // 次の清算を計算
    calculate_repayments(&mut updated_creditors, &mut updated_debtors, repayments);
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
            (1400, "N", vec!["B", "D", "J", "N", "P", "R", "S", "X", "Y"]),
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
