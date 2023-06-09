use super::builtins::get_builtin_functions;
use super::Value;
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn root() -> Self {
        let builtins = get_builtin_functions();

        let mut map = HashMap::new();

        for builtin in builtins {
            map.insert(builtin.name().to_owned(), Value::Builtin(builtin));
        }

        Self { map, parent: None }
    }

    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
            parent: None,
        }
    }

    pub fn set_parent(&mut self, environment: Environment) {
        self.parent.replace(Box::new(environment));
    }

    pub fn take_parent(&mut self) -> Option<Environment> {
        let env = self.parent.take();

        env.map(|e| *e)
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key).or_else(|| {
            if let Some(environment) = &self.parent {
                environment.get(key)
            } else {
                None
            }
        })
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            parent.fmt(f)?;
        }

        let key_width = self
            .map
            .keys()
            .map(std::string::String::len)
            .max()
            .unwrap_or_default();

        let value_width = self
            .map
            .values()
            .map(|v| v.to_string().len())
            .max()
            .unwrap_or_default();

        writeln!(
            f,
            "+-{}-+-{}-+",
            "-".repeat(key_width),
            "-".repeat(value_width)
        )?;

        for (key, value) in &self.map {
            if key.starts_with('_') {
                continue;
            }

            writeln!(
                f,
                "| {key:>key_width$} | {:>value_width$} |",
                value.to_string()
            )?;
        }

        writeln!(
            f,
            "+-{}-+-{}-+",
            "-".repeat(key_width),
            "-".repeat(value_width)
        )?;

        Ok(())
    }
}
