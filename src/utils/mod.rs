const NUMBER_NAMES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS_NAMES: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const TEN_POWER_3N_NAMES: [&str; 3] = ["", "thousand", "million"];

fn two_digits_to_text(n: i32) -> String {
    if n < 20 {
        NUMBER_NAMES[n as usize].to_string()
    } else {
        let u: usize = n.rem_euclid(10) as usize;
        let t: usize = (n / 10) as usize;

        if u == 0 {
            TENS_NAMES[t].to_string()
        } else {
            format!("{} {}", TENS_NAMES[t], NUMBER_NAMES[u])
        }
    }
}

fn three_digits_to_text(n: i32) -> String {
    if n < 100 {
        two_digits_to_text(n)
    } else {
        let h = n / 100;
        let tu = n.rem_euclid(100);
        let mut txt = format!("{} hundred", NUMBER_NAMES[h as usize]);

        if tu > 0 {
            txt.push_str(" and ");
            txt.push_str(&two_digits_to_text(tu));
        }

        txt
    }
}

pub fn number_to_text(n: i32) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        let mut txt_parts: Vec<(i32, String)> = Vec::new();
        let mut top3_digits = n / 1000000;
        let mut remaining_digits = n.rem_euclid(1000000);

        // Break number into three digit chunks
        for ten_power_3n in (0..=2).rev() {
            let mut digits_txt = three_digits_to_text(top3_digits);

            // Do the top three digits need a thousands name?
            if top3_digits > 0 && ten_power_3n > 0 {
                digits_txt.push_str(" ");
                digits_txt.push_str(TEN_POWER_3N_NAMES[ten_power_3n]);

                txt_parts.append(&mut vec![(
                    top3_digits * 10_i32.pow((3 * ten_power_3n) as u32),
                    digits_txt,
                )]);
            }

            // On the last iteration, check that the lowest three digits are processed
            if ten_power_3n == 0 && remaining_digits > 0 {
                txt_parts.append(&mut vec![(
                    remaining_digits,
                    three_digits_to_text(remaining_digits),
                )]);
            }

            // Shunt working values
            top3_digits = remaining_digits / 1000;
            remaining_digits = remaining_digits.rem_euclid(1000);
        }

        let mut number: Vec<String> = vec![String::new()];

        // Construct the number's name inserting the word "and" anytime a given block is < 100
        for (idx, (_n, t)) in txt_parts.iter().enumerate() {
            if !t.is_empty() {
                let idx_next = idx + 1;
                let this_part = if idx_next < txt_parts.len()
                    && txt_parts[idx_next].0 > 0
                    && txt_parts[idx_next].0 < 100
                {
                    format!(" {} and", t)
                } else {
                    format!(" {}", t)
                };

                number.append(&mut vec![this_part])
            }
        }

        number.join("").trim().to_string()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod tests;
