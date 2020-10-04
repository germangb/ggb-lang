macro_rules! parse {
    (
        $(#[$($meta:meta)+])*
        pub struct $ident:ident<'a $(, $gen:ident )*>
        $(
            where
                $($gen_w:ident: $bound_w:ident<'a>,)*
        )?
        {
            $(pub $field:ident: $ty:ty,)*
        }

        // FIXME remove quick hack
        //  make the 'a optional
        $({
            $($phantom_fields:ident: $phantom_ty:ty,)*
        })?
    ) => {
        $(#[$($meta)+])*
        pub struct $ident<'a $(, $gen)*>
        // $(
        //     where
        //         $($gen_w: $bound_w<'a>,)*
        // )?
        {
            $(pub $field: $ty,)*
            $($($phantom_fields: $phantom_ty,)*)?
        }

        //impl<$($gen,)*> crate::span::Spanned for $ident<'_ $(, $gen)*>
        //where
        //    $($gen: crate::span::Spanned,)*
        //{
        //    fn span(&self) -> crate::span::Span {
        //        use crate::span::Spanned;
        //        let mut first = true;
        //        let mut span = None;
        //        $(
        //        let $field = self.$field.span();
        //        if first { span = Some(self.$field.span()); first = false; }
        //        else { span = Some(crate::span::union(&span.unwrap(), &$field)); }
        //        )*
        //        span.unwrap()
        //    }
        //}

        impl<'a $(, $gen/*: crate::ast::Grammar<'a>*/ )*> crate::ast::Grammar<'a> for $ident<'a $(, $gen)*>
        $(
            where
                $($gen_w: $bound_w<'a>,)*
        )?
        {
            fn parse(
                context: &mut crate::ast::Context<'a, '_>,
                tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
            ) -> Result<Self, crate::error::Error<'a>> {
                Ok(Self {
                    $($field: crate::ast::Grammar::parse(context, tokens)?,)*
                    $($($phantom_fields: std::marker::PhantomData,)*)?
                })
            }
        }
    }
}

macro_rules! parse_tuple {
    ($($gen:ident),*) => {
        impl<'a, $($gen: Grammar<'a>),*> crate::ast::Grammar<'a> for ($($gen),*) {
            fn parse(
                context: &mut crate::ast::Context<'a, '_>,
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

        //impl<'a, $($gen: crate::ast::StatementGrammar<'a>),*> crate::ast::StatementGrammar<'a> for ($( $gen ),*) {}
    }
}
