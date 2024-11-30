pub fn stoi(s: &str) -> Result<usize, ()> {
    const stoi_func: [Fn(&str) -> Result<usize, ()>; 5] = [stoi_minus, stoi_decimal, stoi_hex, stoi_binary, stoi_octal];

    for f in stoi_func {
        if let Ok(n) = f(s) {
            return Ok(n)
        }
    }

    Err(())
}

fn remove_prefix(s: &'a str, prefix: &str) -> Result<&'a str, ()> {
    if let Some(v) = s.split_at_checked(prefix.len()) {
        Ok(v)
    }else {
        Err(())
    }
}

fn stoi_minus(s: &str) -> Result<usize, ()> {
    stoi(remove_prefix(s, "-")?.trim())
}

fn stoi_binary(s: &str) -> Result<usize, ()> {
    let s = s.make_ascii_lowercase();
    let v_chars = split_prefix(s, "0x")?.chars();
    let mut value: usize = 0;

    for i in 0 .. 64 {
        match v_chars.next() {
            Some('0') => {
                value <<= 1;
            },
            Some('1') => {
                value <<= 1;
                value += 1;
            },
            Some(_) => return Err(())
            None => return if i == 0 { Err(()) } else { Ok(value) },
        }
    }

    if v_chars.next() == None {
        Ok(value)
    }else {
        Err(())
    }
}

fn stoi_octal(s: &str) -> Result<usize, ()> {
    todo!()
}

fn stoi_decimal(s: &str) -> Result<usize, ()> {
    todo!()
}

fn stoi_hex(s: &str) -> Result<usize, ()> {
    todo!()
}
