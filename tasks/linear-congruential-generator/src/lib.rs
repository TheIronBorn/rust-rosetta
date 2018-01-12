pub struct BsdLcg {
    state: u32,
}

impl BsdLcg {
    pub fn new(seed: u32) -> BsdLcg {
        BsdLcg { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state %= 1 << 31;
        self.state
    }
}

pub struct MsLcg {
    state: u32,
}

impl MsLcg {
    pub fn new(seed: u32) -> MsLcg {
        MsLcg { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(214013).wrapping_add(2531011);
        self.state %= 1 << 31;
        self.state >> 16
    }
}

#[test]
fn test_ms() {
    let mut rng = MsLcg::new(1);

    for &num in &[41, 18467, 6334, 26500, 19169] {
        assert_eq!(rng.next(), num);
    }
}

#[test]
fn test_bsd() {
    let mut rng = BsdLcg::new(1);

    for &num in &[1103527590, 377401575, 662824084, 1147902781, 2035015474] {
        assert_eq!(rng.next(), num);
    }
}
