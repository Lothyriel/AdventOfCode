pub fn get_historical_value(input: &str) -> i64 {
    input.lines().map(get_value).sum()
}

fn get_value(input: &str) -> i64 {
    let history = input.split_whitespace().flat_map(|n| n.parse()).collect();

    predict_value(0, history)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let result = get_value("0 3 6 9 12 15");
        assert_eq!(result, 18);
    }

    #[test]
    fn example() {
        let result = get_historical_value(include_str!("example"));
        assert_eq!(result, 114);
    }

    #[test]
    fn puzzle() {
        let result = get_historical_value(include_str!("input"));
        assert_eq!(result, 0);
    }
}
