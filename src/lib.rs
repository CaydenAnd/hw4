use std::collections::HashMap;

pub fn fib(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    if n == 0 {
        return v;
    }
    v.push(0);
    if n == 1 {
        return v;
    }
    v.push(1);
    for i in 2..n {
        let next = v[(i - 1) as usize] + v[(i - 2) as usize];
        v.push(next);
    }
    v
}

pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}

pub fn nthmax(n: usize, a: &[i32]) -> Option<i32> {
    if a.is_empty() {
        return None;
    }
    let mut v = a.to_vec();
    v.sort_by(|a, b| b.cmp(a));
    v.get(n).cloned()
}

pub fn freq(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut best = (' ', 0);
    for (c, count) in map {
        if count > best.1 {
            best = (c, count);
        }
    }
    best.0.to_string()
}

pub fn zip_hash(
    arr1: &[String],
    arr2: &[String],
) -> Option<HashMap<String, String>> {
    if arr1.len() != arr2.len() {
        return None;
    }
    let mut m = HashMap::new();
    for i in 0..arr1.len() {
        m.insert(arr1[i].clone(), arr2[i].clone());
    }
    Some(m)
}

pub fn hash_to_array(map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> =
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

#[derive(Debug, Default)]
pub struct PhoneBook {
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    pub fn new() -> Self {
        PhoneBook { entries: Vec::new() }
    }

    fn valid_number(num: &str) -> bool {
        let parts: Vec<&str> = num.split('-').collect();
        if parts.len() != 3 {
            return false;
        }
        if parts[0].len() != 3 || parts[1].len() != 3 || parts[2].len() != 4 {
            return false;
        }
        parts.iter().all(|p| p.chars().all(|c| c.is_digit(10)))
    }

    pub fn add(&mut self, name: String, number: String, is_listed: bool) -> bool {
        for e in &self.entries {
            if e.name == name {
                return false;
            }
        }

        if !Self::valid_number(&number) {
            return false;
        }

        if is_listed {
            for e in &self.entries {
                if e.number == number && e.is_listed {
                    return false;
                }
            }
        }

        self.entries.push(PhoneEntry {
            name,
            number,
            is_listed,
        });

        true
    }

    pub fn lookup(&self, name: &str) -> Option<String> {
        for e in &self.entries {
            if e.name == name && e.is_listed {
                return Some(e.number.clone());
            }
        }
        None
    }

    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        for e in &self.entries {
            if e.number == num && e.is_listed {
                return Some(e.name.clone());
            }
        }
        None
    }

    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        let mut v = Vec::new();
        for e in &self.entries {
            if e.number.starts_with(areacode) {
                v.push(e.name.clone());
            }
        }
        v
    }
}
