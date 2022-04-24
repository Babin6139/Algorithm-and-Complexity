#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn knap_sack(p: &[i32], w: i32, m: i32) {
    let n: usize = p.len();
    let max_profit = 0;
    let solution: String = String::from("");
    let global_weight: i32 = 0;
    let bitstring = get_string(&n);
}

fn get_string(n: &usize) {
    let mut i = 0;
    let mut bit_string: Vec<String> = vec![];
    println!("{}", n);
    while i < *n {
        let mut bitstr = format!("0{:b}", i);
        if (*n - bitstr.len() == 0) {
            bit_string.push(bitstr);
        } else {
            let mut j = 0;
            while j < *n - bitstr.len() {
                bitstr = format!("{}{}", "0", bitstr);
                j = j + 1;
            }
            bit_string.push(bitstr);
        }
        i = i + 1;
    }
    println!("{:?}", bit_string);
}
