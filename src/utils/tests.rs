use super::*;

#[test]
fn should_name_single_digit_numbers() {
    assert_eq!("one", two_digits_to_text(1));
    assert_eq!("two", two_digits_to_text(2));
    assert_eq!("three", two_digits_to_text(3));
    assert_eq!("four", two_digits_to_text(4));
    assert_eq!("five", two_digits_to_text(5));
    assert_eq!("six", two_digits_to_text(6));
    assert_eq!("seven", two_digits_to_text(7));
    assert_eq!("eight", two_digits_to_text(8));
    assert_eq!("nine", two_digits_to_text(9));

    assert_eq!("zero", number_to_text(0));
    assert_eq!("one", number_to_text(1));
    assert_eq!("two", number_to_text(2));
    assert_eq!("three", number_to_text(3));
    assert_eq!("four", number_to_text(4));
    assert_eq!("five", number_to_text(5));
    assert_eq!("six", number_to_text(6));
    assert_eq!("seven", number_to_text(7));
    assert_eq!("eight", number_to_text(8));
    assert_eq!("nine", number_to_text(9));
}

#[test]
fn should_name_two_digit_numbers() {
    assert_eq!("ten", two_digits_to_text(10));
    assert_eq!("eleven", two_digits_to_text(11));
    assert_eq!("twelve", two_digits_to_text(12));
    assert_eq!("thirteen", two_digits_to_text(13));
    assert_eq!("fourteen", two_digits_to_text(14));
    assert_eq!("fifteen", two_digits_to_text(15));
    assert_eq!("sixteen", two_digits_to_text(16));
    assert_eq!("seventeen", two_digits_to_text(17));
    assert_eq!("eighteen", two_digits_to_text(18));
    assert_eq!("nineteen", two_digits_to_text(19));
    assert_eq!("twenty", two_digits_to_text(20));
    assert_eq!("twenty one", two_digits_to_text(21));
    assert_eq!("twenty two", two_digits_to_text(22));
    assert_eq!("twenty three", two_digits_to_text(23));
    assert_eq!("twenty four", two_digits_to_text(24));
    assert_eq!("twenty five", two_digits_to_text(25));
    assert_eq!("twenty six", two_digits_to_text(26));
    assert_eq!("twenty seven", two_digits_to_text(27));
    assert_eq!("twenty eight", two_digits_to_text(28));
    assert_eq!("twenty nine", two_digits_to_text(29));
    assert_eq!("thirty", two_digits_to_text(30));
    assert_eq!("fifty", two_digits_to_text(50));
    assert_eq!("fifty one", two_digits_to_text(51));
    assert_eq!("fifty two", two_digits_to_text(52));
    assert_eq!("fifty three", two_digits_to_text(53));
    assert_eq!("fifty four", two_digits_to_text(54));
    assert_eq!("fifty five", two_digits_to_text(55));
    assert_eq!("fifty six", two_digits_to_text(56));
    assert_eq!("fifty seven", two_digits_to_text(57));
    assert_eq!("fifty eight", two_digits_to_text(58));
    assert_eq!("fifty nine", two_digits_to_text(59));
    assert_eq!("ninety", two_digits_to_text(90));
    assert_eq!("ninety one", two_digits_to_text(91));
    assert_eq!("ninety two", two_digits_to_text(92));
    assert_eq!("ninety three", two_digits_to_text(93));
    assert_eq!("ninety four", two_digits_to_text(94));
    assert_eq!("ninety five", two_digits_to_text(95));
    assert_eq!("ninety six", two_digits_to_text(96));
    assert_eq!("ninety seven", two_digits_to_text(97));
    assert_eq!("ninety eight", two_digits_to_text(98));
    assert_eq!("ninety nine", two_digits_to_text(99));

    assert_eq!("ten", number_to_text(10));
    assert_eq!("eleven", number_to_text(11));
    assert_eq!("twelve", number_to_text(12));
    assert_eq!("thirteen", number_to_text(13));
    assert_eq!("fourteen", number_to_text(14));
    assert_eq!("fifteen", number_to_text(15));
    assert_eq!("sixteen", number_to_text(16));
    assert_eq!("seventeen", number_to_text(17));
    assert_eq!("eighteen", number_to_text(18));
    assert_eq!("nineteen", number_to_text(19));
    assert_eq!("twenty", number_to_text(20));
    assert_eq!("twenty one", number_to_text(21));
    assert_eq!("twenty two", number_to_text(22));
    assert_eq!("twenty three", number_to_text(23));
    assert_eq!("twenty four", number_to_text(24));
    assert_eq!("twenty five", number_to_text(25));
    assert_eq!("twenty six", number_to_text(26));
    assert_eq!("twenty seven", number_to_text(27));
    assert_eq!("twenty eight", number_to_text(28));
    assert_eq!("twenty nine", number_to_text(29));
    assert_eq!("thirty", number_to_text(30));
    assert_eq!("fifty", number_to_text(50));
    assert_eq!("fifty one", number_to_text(51));
    assert_eq!("fifty two", number_to_text(52));
    assert_eq!("fifty three", number_to_text(53));
    assert_eq!("fifty four", number_to_text(54));
    assert_eq!("fifty five", number_to_text(55));
    assert_eq!("fifty six", number_to_text(56));
    assert_eq!("fifty seven", number_to_text(57));
    assert_eq!("fifty eight", number_to_text(58));
    assert_eq!("fifty nine", number_to_text(59));
    assert_eq!("ninety", number_to_text(90));
    assert_eq!("ninety one", number_to_text(91));
    assert_eq!("ninety two", number_to_text(92));
    assert_eq!("ninety three", number_to_text(93));
    assert_eq!("ninety four", number_to_text(94));
    assert_eq!("ninety five", number_to_text(95));
    assert_eq!("ninety six", number_to_text(96));
    assert_eq!("ninety seven", number_to_text(97));
    assert_eq!("ninety eight", number_to_text(98));
    assert_eq!("ninety nine", number_to_text(99));
}

#[test]
fn should_name_three_digit_numbers() {
    assert_eq!("ninety nine", three_digits_to_text(99));
    assert_eq!("one hundred", three_digits_to_text(100));
    assert_eq!("one hundred and one", three_digits_to_text(101));
    assert_eq!("one hundred and ten", three_digits_to_text(110));
    assert_eq!("one hundred and twenty", three_digits_to_text(120));
    assert_eq!("one hundred and twenty seven", three_digits_to_text(127));
    assert_eq!("four hundred and fifty nine", three_digits_to_text(459));
    assert_eq!("nine hundred", three_digits_to_text(900));

    assert_eq!("ninety nine", number_to_text(99));
    assert_eq!("one hundred", number_to_text(100));
    assert_eq!("one hundred and one", number_to_text(101));
    assert_eq!("one hundred and ten", number_to_text(110));
    assert_eq!("one hundred and twenty", number_to_text(120));
    assert_eq!("one hundred and twenty seven", number_to_text(127));
    assert_eq!("four hundred and fifty nine", number_to_text(459));
    assert_eq!("nine hundred", number_to_text(900));
}

#[test]
fn should_name_four_digit_numbers() {
    assert_eq!("one thousand", number_to_text(1000));
    assert_eq!("one thousand and one", number_to_text(1001));
    assert_eq!("one thousand and ninety nine", number_to_text(1099));
    assert_eq!(
        "one thousand one hundred and ninety nine",
        number_to_text(1199)
    );
    assert_eq!(
        "seventeen thousand six hundred and two",
        number_to_text(17602)
    );
    assert_eq!(
        "ninety seven thousand eight hundred and thirty five",
        number_to_text(97835)
    );
}

#[test]
fn should_name_five_digit_numbers() {
    assert_eq!("ten thousand", number_to_text(10000));
    assert_eq!("ten thousand and one", number_to_text(10001));
    assert_eq!("ten thousand one hundred and one", number_to_text(10101));
    assert_eq!(
        "twelve thousand three hundred and forty five",
        number_to_text(12345)
    );
    assert_eq!(
        "eighty one thousand seven hundred and twenty six",
        number_to_text(81726)
    );
    assert_eq!(
        "ninety nine thousand nine hundred and ninety nine",
        number_to_text(99999)
    );
}

#[test]
fn should_name_six_digit_numbers() {
    assert_eq!("one hundred thousand", number_to_text(100000));
    assert_eq!("one hundred thousand and one", number_to_text(100001));
    assert_eq!(
        "one hundred thousand one hundred and one",
        number_to_text(100101)
    );
    assert_eq!(
        "one hundred and twenty three thousand four hundred and fifty six",
        number_to_text(123456)
    );
    assert_eq!(
        "seven hundred and sixteen thousand two hundred and fifty three",
        number_to_text(716253)
    );
    assert_eq!(
        "nine hundred and ninety nine thousand nine hundred and ninety nine",
        number_to_text(999999)
    );
}

#[test]
fn should_name_seven_digit_numbers() {
    assert_eq!("one million", number_to_text(1000000));
    assert_eq!("one million and one", number_to_text(1000001));
    assert_eq!("one million one hundred and one", number_to_text(1000101));
    assert_eq!("one million one thousand and one", number_to_text(1001001));
    assert_eq!(
        "one million two hundred and thirty four thousand five hundred and sixty seven",
        number_to_text(1234567)
    );
    assert_eq!(
        "nine million one hundred and eighty two thousand seven hundred and thirty six",
        number_to_text(9182736)
    );
    assert_eq!(
        "nine million nine hundred and ninety nine thousand nine hundred and ninety nine",
        number_to_text(9999999)
    );
}

#[test]
fn should_name_eight_digit_numbers() {
    assert_eq!("ten million", number_to_text(10000000));
    assert_eq!("ten million and one", number_to_text(10000001));
    assert_eq!("ten million one hundred and one", number_to_text(10000101));
    assert_eq!("ten million one thousand and one", number_to_text(10001001));
    assert_eq!(
        "twelve million three hundred and forty five thousand six hundred and seventy eight",
        number_to_text(12345678)
    );
    assert_eq!(
        "ninety one million eight hundred and twenty seven thousand three hundred and sixty four",
        number_to_text(91827364)
    );
    assert_eq!(
        "ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine",
        number_to_text(99999999)
    );
}

#[test]
fn should_name_nine_digit_numbers() {
    assert_eq!("one hundred million", number_to_text(100000000));
    assert_eq!("one hundred million and one", number_to_text(100000001));
    assert_eq!(
        "one hundred million one hundred and one",
        number_to_text(100000101)
    );
    assert_eq!(
        "one hundred million one thousand and one",
        number_to_text(100001001)
    );
    assert_eq!(
        "one hundred and twenty three million four hundred and fifty six thousand seven hundred and eighty nine",
        number_to_text(123456789)
    );
    assert_eq!(
        "nine hundred and eighteen million two hundred and seventy three thousand six hundred and forty five",
        number_to_text(918273645)
    );
    assert_eq!(
        "nine hundred and ninety nine million nine hundred and ninety nine thousand nine hundred and ninety nine",
        number_to_text(999999999)
    );
}
