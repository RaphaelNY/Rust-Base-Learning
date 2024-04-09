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

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}