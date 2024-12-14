#[derive(Clone, Copy, Debug)]
pub struct Matrix2x2 {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
}

impl Matrix2x2 {
    pub const fn new(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self { a, b, c, d }
    }

    pub fn det(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }
}

impl std::fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "⧚{:0<5} {:0<5}⧚", self.a, self.b)?;
        writeln!(f, "⧚{:0<5} {:0<5}⧚\n", self.c, self.d)
    }
}
