fn main() {
    println!("DNA to RNA");
}

fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(|c| match c {
        'T' => 'U',
        _ => c
    }).collect()
    // dna.replace("T", "U") // Alternative solution
}

fn make_upper_case(s: &str) -> String {
    s.to_ascii_uppercase()
    // static make_upper_case: fn(&str) -> String = str::to_uppercase;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }
}