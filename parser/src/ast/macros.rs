macro_rules! span {
    ($ident:ident { $head:ident $(, $tail:ident)* }) => {
        impl $crate::lex::span::Spanned for $ident<'_> {
            fn span(&self) -> $crate::lex::span::Span {
                #[allow(unused)]
                let mut union = self.$head.span();
                $(union = $crate::lex::span::union(&union, &self.$tail.span());)*
                union
            }
        }
    };
    // TODO rework
    ($ident:ident<I> { $head:ident $(, $tail:ident)* }) => {
        impl<I> $crate::lex::span::Spanned for $ident<'_, I> {
            fn span(&self) -> $crate::lex::span::Span {
                #[allow(unused)]
                let mut union = self.$head.span();
                $(union = $crate::lex::span::union(&union, &self.$tail.span());)*
                union
            }
        }
    }
}

macro_rules! parse {
    // enum parsing
    ($(#[$($meta:meta)+])*
     pub enum $enum_name:ident<'a> {
         $( $(#[$($var_meta:meta)+])*
            $var_name:ident ( $var_type:ty ) ,)*
    }) => {
        $(#[$($meta)+])*
        pub enum $enum_name<'a> {
            $( $(#[$($var_meta)+])* $var_name( $var_type ) ,)*
        }

        impl $crate::lex::span::Spanned for $enum_name<'_> {
            fn span(&self) -> crate::lex::span::Span {
                match self { $($enum_name::$var_name(s) => s.span(),)* }
            }
        }
    };

    // struct parsing
    ($(#[$($meta:meta)+])*
     pub struct $ident:ident<'a> {
         $( $(#[$field_meta:meta])* pub $field:ident: $ty:ty, )*
     }
     $({ $($phantom_fields:ident: $phantom_ty:ty,)* })?
    ) => {
        $(#[$($meta)+])*
        pub struct $ident<'a> {
            $( $(#[$field_meta])* pub $field: $ty, )*
            $( $($phantom_fields: $phantom_ty,)* )?
        }

        impl<'a> crate::ast::Grammar<'a> for $ident<'a> {
            fn parse(context: &mut crate::ast::Context<'a>,
                     tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>)
                     -> Result<Self, crate::error::Error<'a>> {
                Ok(Self { $($field: crate::ast::Grammar::parse(context, tokens)?,)*
                          $($($phantom_fields: std::marker::PhantomData,)*)? })
            }
        }
    };
}
