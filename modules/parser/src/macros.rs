macro_rules! parse {
    (
        $(#[$($meta:meta)+])*
        pub struct $ident:ident<'a $(, $gen:ident )*> {
            $(pub $field:ident: $ty:ty,)*
        }
    ) => {
        $(#[$($meta)+])*
        pub struct $ident<'a $(, $gen)*> {
            $(pub $field: $ty,)*
        }

        impl<'a $(, $gen: crate::ast::Grammar<'a>)*> crate::ast::Grammar<'a> for $ident<'a $(, $gen)*> {
            fn parse(
                context: &mut crate::ast::Context,
                tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::error::Error<'a>> {
                Ok(Self {
                    $($field: crate::ast::Grammar::parse(context, tokens)?,)*
                })
            }
        }
    }
}

macro_rules! parse_vec_separated {
    ($foo:ty, $bar:ty) => {
        impl<'a> crate::ast::Parse<'a> for Vec<($foo, $bar)> {
            fn parse(
                context: &mut crate::ast::Context,
                tokens: &mut Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::ast::Error<'a>> {
                let mut vec = Vec::new();
                while let Some(foo) = crate::ast::Parse::parse(context, tokens)? {
                    let bar = crate::ast::Parse::parse(context, tokens)?;
                    vec.push((foo, bar));
                }
                Ok(vec)
            }
        }
    };
}

macro_rules! parse_tuple {
    ($($gen:ident),*) => {
        impl<'a, $($gen: Grammar<'a>),*> crate::ast::Grammar<'a> for ($($gen),*) {
            fn parse(
                context: &mut crate::ast::Context,
                tokens: &mut Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::ast::Error<'a>> {
                Ok((
                    $({
                        let meow: $gen = crate::ast::Grammar::parse(context, tokens)?;
                        meow
                    }),*
                ))
            }
        }

        impl<'a, $($gen: crate::ast::StatementGrammar<'a>),*> crate::ast::StatementGrammar<'a> for ($( $gen ),*) {}
    }
}
