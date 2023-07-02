use super::{effect::Effect, target::Target};

#[derive(Debug)]
pub struct Skill {
    name: String,
    effect: Effect,
    target: Target,
}

impl Skill {
    pub fn new(name: String, effect: Effect, target: Target) -> Self {
        Self {
            name,
            effect,
            target,
        }
    }

    pub fn damage(name: &str, eq: &str, target: Target) -> Self {
        Self::new(name.to_string(), Effect::Damage(eq.to_string()), target)
    }

    pub fn perform(&self) -> Result<(), ()> {
        println!(
            "You perform {name} on {target}, dealing {effect} damage",
            name = self.name,
            effect = self.effect.roll(),
            target = "enemy"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::systems::target::Target;

    use super::Skill;

    #[test]
    fn test_single_damage_skill() {
        let skill = Skill::damage("Slash", "1d6", Target::Enemy);
        assert!(skill.perform().is_ok())
    }
}
