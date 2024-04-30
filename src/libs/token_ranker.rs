use std::collections::HashMap;

pub fn rank_tokens(tokens: Vec<String>) -> Vec<(String, i32)> {
    let mut frequency: HashMap<String, i32> = HashMap::new();
    for token in tokens {
        *frequency.entry(token).or_insert(0) += 1;
    }

    let mut ranking: Vec<(String, i32)> = frequency.into_iter().collect();
    ranking.sort_by(|a, b| {
        let freq_comparison = b.1.cmp(&a.1);
        if freq_comparison == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            freq_comparison
        }
    });

    ranking
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_tokens() {
        let tokens = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let result = rank_tokens(tokens);
        assert_eq!(result, vec![("a".to_string(), 2), ("b".to_string(), 1)]);
    }
}
