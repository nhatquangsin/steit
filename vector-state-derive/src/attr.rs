use crate::context::Context;

pub struct Attr<'a, T> {
    context: &'a Context,
    name: &'static str,
    value: Option<T>,
    tokens: proc_macro2::TokenStream,
}

impl<'a, T> Attr<'a, T> {
    pub fn new(context: &'a Context, name: &'static str) -> Self {
        Self {
            context,
            name,
            value: None,
            tokens: proc_macro2::TokenStream::new(),
        }
    }

    pub fn set<O: quote::ToTokens>(&mut self, object: O, value: T) {
        let tokens = object.into_token_stream();

        if self.value.is_some() {
            self.context
                .error(tokens, format!("duplicate state attribute `{}`", self.name));
        } else {
            self.value = Some(value);
            self.tokens = tokens;
        }
    }

    pub fn value(self) -> Option<AttrValue<T>> {
        let Attr { value, tokens, .. } = self;
        value.map(|value| AttrValue { value, tokens })
    }
}

pub struct AttrValue<T> {
    value: T,
    tokens: proc_macro2::TokenStream,
}

impl<T> AttrValue<T> {
    pub fn get(&self) -> &T {
        &self.value
    }
}

impl<T> quote::ToTokens for AttrValue<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}