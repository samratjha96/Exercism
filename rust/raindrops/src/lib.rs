pub fn raindrops(n: u32) -> String {
    let is_factor = |factor| n % factor == 0;
    let mut ret = String::new();

    if is_factor(3) { ret.push_str("Pling"); }
    if is_factor(5) { ret.push_str("Plang"); }
    if is_factor(7) { ret.push_str("Plong"); }

    if ret.is_empty() { ret = n.to_string(); }

    ret
}

