use std::collections::{HashMap};
use crate::http::query_string::Value::{Multiple, Single};

#[derive(Debug)]
pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>
}

#[derive(Debug)]
pub enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>)
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(value: &'buffer str) -> Self {
        let mut data: HashMap<&'buffer str, Value<'buffer>> = HashMap::new();

        for binding in value.split('&') {
            let mut key = binding;
            let mut value = "";
            if let Some(index) = binding.find('=') {
                key = &binding[..index];
                value = &binding[index+1..];
            }

            data.entry(key).and_modify( |mut existing|
                match existing {
                    Single(existing_value) => {
                        let mut new_item: Vec<&str> = Vec::new();
                        new_item.push(existing_value);
                        new_item.push(value);
                        *existing = Multiple(new_item);
                    }
                    Multiple(existing_list) => {
                        existing_list.push(value);
                    }

                }
            ).or_insert(Single(value));
        }


        QueryString {
            data
        }
    }
}