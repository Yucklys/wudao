use dice_forge::Equation;

pub fn roll_with_eq(eq: &str) -> i32 {
    let equation = Equation::new(eq).unwrap();
    equation.roll().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dice_roll() {
        let result = roll_with_eq("1d6+3");
        let possible_values = 4..=9;
        assert!(possible_values.contains(&result))
    }
}
