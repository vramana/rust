#![crate_name = "foo"]


pub trait Foo {
    // @has foo/trait.Foo.html '//h3[@id="tymethod.foo"]//div[@class="code-attribute"]' '#[must_use]'
    #[must_use]
    fn foo();
}

#[must_use]
pub struct Bar;

impl Bar {
    // @has foo/struct.Bar.html '//h4[@id="method.bar"]//div[@class="code-attribute"]' '#[must_use]'
    #[must_use]
    pub fn bar() {}

    // @has foo/struct.Bar.html '//h4[@id="method.bar2"]//div[@class="code-attribute"]' '#[must_use]'
    #[must_use]
    pub fn bar2() {}
}
