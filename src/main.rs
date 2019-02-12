mod components {
    use foobar::*;

    #[derive(Clone)]
    pub struct Foo {
        pub bar: Bar,
    }
}

fn main() {
    use self::components::*;
    use foobar::Bar;

    Foo { bar: Bar };
}
