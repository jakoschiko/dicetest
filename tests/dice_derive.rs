use dicetest::{Limit, prelude::*};

#[test]
fn struct_has_expected_size() {
    #[derive(Dice)]
    struct Foo {
        _dummy1: bool,
        data1: Vec<u8>,
        _dummy2: bool,
        data2: Vec<u8>,
        _dummy3: bool,
    }

    impl Foo {
        fn size(&self) -> usize {
            self.data1.len() + self.data2.len()
        }
    }

    Dicetest::repeatedly().run(|mut fate| {
        let size = fate.roll(dice::length(..));
        let limit = Limit::saturating_from_usize(size);
        let foo: Foo = fate.with_limit(limit).roll(die());
        assert!(foo.size() <= size);
    })
}

#[test]
fn enum_has_expected_size() {
    #[derive(Dice)]
    enum Foo {
        A(Vec<u8>),
        B(Vec<u8>),
    }

    impl Foo {
        fn size(&self) -> usize {
            match self {
                Self::A(data) => data.len(),
                Self::B(data) => data.len(),
            }
        }
    }

    Dicetest::repeatedly().run(|mut fate| {
        let size = fate.roll(dice::length(..));
        let limit = Limit::saturating_from_usize(size);
        let foo: Foo = fate.with_limit(limit).roll(die());
        assert!(foo.size() <= size);
    })
}

#[test]
fn weighted_enum_has_expected_size() {
    #[derive(Dice)]
    enum Foo {
        A(Vec<u8>),
        #[dice(weight = 2)]
        B(Vec<u8>),
    }

    impl Foo {
        fn size(&self) -> usize {
            match self {
                Self::A(data) => data.len(),
                Self::B(data) => data.len(),
            }
        }
    }

    Dicetest::repeatedly().run(|mut fate| {
        let size = fate.roll(dice::length(..));
        let limit = Limit::saturating_from_usize(size);
        let foo: Foo = fate.with_limit(limit).roll(die());
        assert!(foo.size() <= size);
    })
}

#[test]
fn weighted_enum_omit_variants_with_zero_weight() {
    #[derive(Debug, Dice, PartialEq)]
    enum Foo {
        A,
        #[dice(weight = 2)]
        B,
        #[dice(weight = 0)]
        C,
    }

    Dicetest::repeatedly().run(|mut fate| {
        let size = fate.roll(dice::length(..));
        let limit = Limit::saturating_from_usize(size);
        let foo: Foo = fate.with_limit(limit).roll(die());
        assert_ne!(foo, Foo::C);
    })
}

#[test]
fn custom_die_is_used() {
    #[derive(Dice)]
    struct Foo {
        #[dice(die = dice::just(true))]
        x: bool,
    }

    Dicetest::repeatedly().run(|mut fate| {
        let foo: Foo = fate.roll(die());
        assert!(foo.x);
    })
}

#[test]
fn struct_calc_stats() {
    #[derive(Dice)]
    struct Foo {
        data1: Vec<u8>,
        data2: Vec<u8>,
        data3: Vec<u8>,
    }

    Dicetest::repeatedly()
        .passes(0)
        .stats_enabled(true)
        .run(|mut fate| {
            let foo: Foo = fate.with_limit(Limit(3)).roll(die());
            stat!(
                "len of (data1, data2, data3)",
                "{:?}",
                (foo.data1.len(), foo.data2.len(), foo.data3.len())
            );
        })
}

#[test]
fn weighted_enum_calc_stats() {
    #[derive(Debug, Dice)]
    enum Foo {
        A,
        #[dice(weight = 2)]
        B,
        #[dice(weight = 0)]
        C,
        D,
    }

    Dicetest::repeatedly()
        .passes(0)
        .stats_enabled(true)
        .run(|mut fate| {
            let foo: Foo = fate.roll(die());
            stat_debug!(foo);
        })
}
