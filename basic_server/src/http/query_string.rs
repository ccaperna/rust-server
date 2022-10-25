use std::collections::HashMap;


// a=1&b=2&c&d=&e===&d=7&d=abc

pub struct QueryString<'buf>{

    data: HashMap<&'buf str, Value<'buf>>,

}

pub enum Value<'buf> {

    Single(&'buf str),
    //vector is a heap (increases size dinamically)
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {

    fn from(s: &'buf str) -> Self {

        let mut data = HashMap::new();

        for sub_str in s.split('&') {

            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {

                key = &sub_str[..i];
                //i+1 is ok because "=" is only 1 byte
                val = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing{
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec) => vec.push(val),
            })
            .or_insert(Value::Single(val));

        }
        //placed in the first place so the compiler knows
        //the hashmap types
        QueryString{ data }


    }

}