use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

impl<'buf> Headers<'buf> {
    pub fn get(&self, key: &str) -> Option<&str> {
        match self.data.get(key) {
            Some(v) => Some(*v),
            None => None
        }
    }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(str: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub in str.split("\r\n") {
            let mut key = sub;
            let mut val = "";

            if let Some(i) = sub.find(':') {
                key = &sub[..i];
                val = &sub[i + 1..].trim();
            }

            data.insert(key, val);
        }

        Headers { data }
    }
}

impl<'buf> ToString for Headers<'buf> {
    fn to_string(&self) -> String {
        let mut str = String::with_capacity(4096);
        for (k, v) in self.data.iter() {
            for k_char in k.chars() {
                str.push(k_char);
            }
            str.push(':');
            str.push(' ');
            for v_char in v.chars() {
                str.push(v_char);
            }
            str.push('\r');
            str.push('\n');
        }
        str
    }
}
