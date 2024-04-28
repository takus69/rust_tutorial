struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter{ count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // (1, 2), (2, 3), (3, 4), (4, 5)のうち掛けて3の倍数のみを合計
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                .map(|(a, b)| a * b)
                                .filter(|x| x % 3 == 0)
                                .sum();
    assert_eq!(18, sum);
}