use std::{cell::RefCell, fmt, thread};

pub struct Context {
    errors: RefCell<Option<Vec<syn::Error>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn error<O: quote::ToTokens, M: fmt::Display>(
        &self,
        object: O,
        message: M,
    ) -> proc_macro2::TokenStream {
        self.syn_error(syn::Error::new_spanned(object.to_token_stream(), message))
    }

    pub fn syn_error(&self, error: syn::Error) -> proc_macro2::TokenStream {
        self.errors
            .borrow_mut()
            .as_mut()
            .unwrap_or_else(|| unreachable!("expected list of errors to be defined"))
            .push(error);

        quote!()
    }

    pub fn check(self) -> Result<(), Vec<syn::Error>> {
        let errors = self
            .errors
            .borrow_mut()
            .take()
            .unwrap_or_else(|| unreachable!("expected list of errors to be defined"));

        match errors.len() {
            0 => Ok(()),
            _ => Err(errors),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}