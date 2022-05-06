use std::{
    collections::HashMap,
};

use anyhow::{Context, Result};
use derive_more::Constructor;

#[derive(Constructor)]
/// TODO: OsString
pub struct Args{
    map: HashMap<String, String>
}

impl Args {
    fn get<'a, 'b>(&'b self, key: &'a str) -> Result<ArgValue<'a, 'b>> {
        let value = self
            .map
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
