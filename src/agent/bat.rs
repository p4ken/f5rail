use std::{
    collections::HashMap,
};

use anyhow::{Context, Result};


/// TODO: OsString
pub struct Args(HashMap<String, String>);

impl Args {
    fn get<'a, 'b>(&'b self, key: &'a str) -> Result<ArgValue<'a, 'b>> {
        let value = self
            .0
            .get(key)
            .with_context(|| format!("{}を指定してください", key))?;
        Ok(ArgValue(key, value))
    }
}

struct ArgValue<'a, 'b>(&'a str, &'b str);

impl<'a, 'b> ArgValue<'a, 'b> {
    fn str(&self) -> &str {
        self.1
    }
}
