use super::dice;

#[derive(Debug)]
pub enum Effect {
    Damage(String),
}

impl Effect {
    pub fn roll(&self) -> i32 {
        match self {
            Effect::Damage(eq) => dice::roll_with_eq(eq),
        }
    }
}
