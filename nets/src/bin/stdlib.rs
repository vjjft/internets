use internets_nets::*;

interactions! {
  pub type Std;

  struct Clone(-U64, +U64, +U64);
  struct Erase(-U64);

  struct False(+Bool);
  struct True(+Bool);

  struct U64(+U64, $u64);

  struct Add(-U64, -U64, +U64);
  struct Sub(-U64, -U64, +U64);
  struct Mul(-U64, -U64, +U64);
  struct Mod(-U64, -U64, +U64);

  struct AddX(-U64, +U64, $u64);
  struct SubX(-U64, +U64, $u64);
  struct MulX(-U64, +U64, $u64);
  struct ModX(-U64, +U64, $u64);

  impl Add(_, i, o) for U64(_, $n) { AddX(i, o, $n) }
  impl Sub(_, i, o) for U64(_, $n) { SubX(i, o, $n) }
  impl Mul(_, i, o) for U64(_, $n) { MulX(i, o, $n) }
  impl Mod(_, i, o) for U64(_, $n) { ModX(i, o, $n) }

  impl AddX(_, o, $x) for U64(_, $y) { U64(o, $x.wrapping_add(y)) }
  impl SubX(_, o, $x) for U64(_, $y) { U64(o, $x.wrapping_sub(y)) }
  impl MulX(_, o, $x) for U64(_, $y) { U64(o, $x.wrapping_mul(y)) }
  impl ModX(_, o, $x) for U64(_, $y) { U64(o, $y % x) }

  struct Gt(-U64, -U64, +Bool);
  struct Lt(-U64, -U64, +Bool);
  struct Eq(-U64, -U64, +Bool);
  struct Ge(-U64, -U64, +Bool);
  struct Le(-U64, -U64, +Bool);

  struct GtX(-U64, +Bool, $u64);
  struct LtX(-U64, +Bool, $u64);
  struct EqX(-U64, +Bool, $u64);
  struct GeX(-U64, +Bool, $u64);
  struct LeX(-U64, +Bool, $u64);

  impl Gt(_, i, o) for U64(_, $n) { GtX(i, o, $n) }
  impl Lt(_, i, o) for U64(_, $n) { LtX(i, o, $n) }
  impl Eq(_, i, o) for U64(_, $n) { EqX(i, o, $n) }
  impl Ge(_, i, o) for U64(_, $n) { GeX(i, o, $n) }
  impl Le(_, i, o) for U64(_, $n) { LeX(i, o, $n) }

  impl GtX(_, o, $x) for U64(_, $y) if (x > y) { True(o) }
  impl LtX(_, o, $x) for U64(_, $y) if (x < y) { True(o) }
  impl EqX(_, o, $x) for U64(_, $y) if (x == y) { True(o) }
  impl GeX(_, o, $x) for U64(_, $y) if (x >= y) { True(o) }
  impl LeX(_, o, $x) for U64(_, $y) if (x <= y) { True(o) }

  impl GtX(_, o, $_) for U64(_, $_) { False(o) }
  impl LtX(_, o, $_) for U64(_, $_) { False(o) }
  impl EqX(_, o, $_) for U64(_, $_) { False(o) }
  impl GeX(_, o, $_) for U64(_, $_) { False(o) }
  impl LeX(_, o, $_) for U64(_, $_) { False(o) }

  impl Clone(_, o1, o2) for U64(_, $n) {
    U64(o1, $n)
    U64(o2, $n)
  }
  impl Erase(_) for U64(_, $_) {}

  struct Print(-U64);

  impl Print(_) for U64(_, $n) if { println!("{}", n); true } {}
  impl Print(_) for U64(_, $_) {}
}

#[allow(unused)]
fn main() {}
