use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}

/// Create a fruit salad by repeating the provided fruits up to `num` items.
pub fn create_fruit_salad_with_fruits(
    num: usize,
    fruits: &[String],
) -> Result<Vec<String>, String> {
    if num > fruits.len() {
        return Err(format!(
            "Requested {} fruits, but only {} unique fruits provided.",
            num,
            fruits.len()
        ));
    }
    let mut salad = fruits.to_vec();
    salad.truncate(num);
    Ok(salad)
}
