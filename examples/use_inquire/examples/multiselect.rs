use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};

fn main() {
    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermemlon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let validator = |a: &[ListOption<&&str>]| {
        if a.len() < 2 {
            return Ok(Validation::Invalid("This list is too small!".into()));
        }

        let x = a.iter().any(|o| *o.value == "Pineapple");

        match x {
            true => Ok(Validation::Valid),
            false => Ok(Validation::Invalid("Remember to buy pineapple".into())),
        }
    };

    //let formatter: MultiOptionFormatter<'_, &str> =
    //    &|a| format!("Selected {} fruits: {a:?}", a.len());
    let formatter: MultiOptionFormatter<'_, &str> = &|a| {
        let selected_fruits: Vec<_> = a.iter().map(|o| *o.value).collect();
        format!(
            "You selected {} fruits: {}",
            a.len(),
            selected_fruits.join(", ")
        )
    };

    let ans = MultiSelect::new("Select the fruits for your shopping list:", options)
        .with_validator(validator)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(_) => println!("I'll get right on it"),
        Err(_) => println!("The shopping list could not be processed"),
    }
}
