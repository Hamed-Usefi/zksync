use ff::{Field, PrimeField, PrimeFieldRepr, BitIterator};

// TODO: replace Vec with Iterator?

pub trait GetBits {
    fn get_bits_le(&self) -> Vec<bool>;
}

pub trait GetBitsFixed {

    /// Get exactly `n` bits from the value in little endian order
    /// If `n` is larger than value bit length, it is padded with `false`
    /// for the result to exactly match `n`
    fn get_bits_le_fixed(&self, n: usize) -> Vec<bool>;
}

impl<Fr: PrimeField> GetBitsFixed for Fr {

    fn get_bits_le_fixed(&self, n: usize) -> Vec<bool> {
        let mut r: Vec<bool> = Vec::with_capacity(n);
        r.extend(BitIteratorLe::new(self.into_repr()).take(n));
        let len = r.len();
        r.extend((len..n).map(|_| false));
        r
    }
}

#[test]
fn test_get_bits() {
    use pairing::bn256::{Fr};

    // 12 = b1100, 3 lowest bits in little endian encoding are: 0, 0, 1.
    let bits = Fr::from_str("12").unwrap().get_bits_le_fixed(3);
    assert_eq!(bits, vec![false, false, true]);

    let bits = Fr::from_str("0").unwrap().get_bits_le_fixed(512);
    assert_eq!(bits, vec!(false; 512));
}

//

// Resulting iterator is little endian: lowest bit first

#[derive(Debug)]
pub struct BitIteratorLe<E> {
    t: E,
    n: usize,
    len: usize,
}

impl<E: AsRef<[u64]>> BitIteratorLe<E> {
    pub fn new(t: E) -> Self {
        let len = t.as_ref().len() * 64;

        BitIteratorLe { t, n: 0, len }
    }
}

impl<E: AsRef<[u64]>> Iterator for BitIteratorLe<E> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.n == self.len {
            None
        } else {
            let part = self.n / 64;
            let bit = self.n - (64 * part);
            self.n += 1;

            Some(self.t.as_ref()[part] & (1 << bit) > 0)
        }
    }
}

#[test]
fn test_bit_iterator_e() {
    let test_vector = [0xa953d79b83f6ab59, 0x6dea2059e200bd39];
    let mut reference: Vec<bool> = BitIterator::new(&test_vector).collect();
    reference.reverse();
    let out: Vec<bool> = BitIteratorLe::new(&test_vector).collect();
    assert_eq!(reference, out);
}