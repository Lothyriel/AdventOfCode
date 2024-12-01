pub mod part_1;
pub mod part_2;

fn predict_value(mut prediction: i64, history: Vec<i64>) -> i64 {
    prediction += history.last().expect("Should contain one value");

    let new: Vec<_> = history
        .iter()
        .zip(history.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    if new.iter().all(|&v| v == 0) {
        return prediction;
    }

    predict_value(prediction, new)
}
