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

        impl<'a $(, $gen: crate::ast::Parse<'a>)*> crate::ast::Parse<'a> for $ident<'a $(, $gen)*> {
            fn parse(
                context: &mut crate::ast::Context,
                tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::ast::Error> {
                Ok(Self {
                    $($field: crate::ast::Parse::parse(context, tokens)?,)*
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
            ) -> Result<Self, crate::ast::Error> {
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
        impl<'a, $($gen: Parse<'a>),*> crate::ast::Parse<'a> for ($($gen),*) {
            fn parse(
                context: &mut crate::ast::Context,
                tokens: &mut Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::ast::Error> {
                Ok((
                    $({
                        let meow: $gen = crate::ast::Parse::parse(context, tokens)?;
                        meow
                    }),*
                ))
            }
        }

        impl<'a, $($gen: crate::ast::ParseType<'a>),*> crate::ast::ParseFields<'a> for ($( crate::ast::Field<'a, $gen> ),*) {}
        impl<'a, $($gen: crate::ast::ParseStatement<'a>),*> crate::ast::ParseStatement<'a> for ($( $gen ),*) {}
    }
}
