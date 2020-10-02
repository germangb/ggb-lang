//! Assembly grammars.
use crate::{
    ast::{Context, Grammar},
    error::Error,
    lex,
    lex::Tokens,
};
use std::iter::Peekable;

pub trait AsmGrammar<'a>: Grammar<'a> {}

macro_rules! asm {
    (
        pub enum $enum:ident<'a> {
            Label(Label<'a>),
            $($variant:ident( $type:ty ),)*
        }
    ) => {
        $(impl<'a> AsmGrammar<'a> for $type {})*


        $(
            #[allow(non_camel_case_types)]
            pub type $variant<'a> = $type;
        )*

        #[allow(non_camel_case_types)]
        pub enum $enum<'a> {
            Label(Label<'a>),
            $($variant( $variant<'a> ),)*
        }
    }
}

asm! {
    pub enum Asm<'a> {
        // <ident> :
        Label(Label<'a>),
        // ld %a, <src>
        Ld_A_B(Ld<'a, lex::A<'a>, lex::B<'a>>),
        Ld_A_C(Ld<'a, lex::A<'a>, lex::C<'a>>),
        Ld_A_D(Ld<'a, lex::A<'a>, lex::D<'a>>),
        Ld_A_E(Ld<'a, lex::A<'a>, lex::E<'a>>),
        Ld_A_H(Ld<'a, lex::A<'a>, lex::L<'a>>),
        Ld_A_HLptr(Ld<'a, lex::A<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_A_d8(Ld<'a, lex::A<'a>, lex::Lit<'a>>),
        Ld_A_a16ptr(Ld<'a, lex::A<'a>, Ptr<'a, lex::Lit<'a>>>),
        // ld %b, <src>
        Ld_B_A(Ld<'a, lex::B<'a>, lex::A<'a>>),
        Ld_B_B(Ld<'a, lex::B<'a>, lex::B<'a>>),
        Ld_B_C(Ld<'a, lex::B<'a>, lex::C<'a>>),
        Ld_B_D(Ld<'a, lex::B<'a>, lex::D<'a>>),
        Ld_B_E(Ld<'a, lex::B<'a>, lex::E<'a>>),
        Ld_B_H(Ld<'a, lex::B<'a>, lex::L<'a>>),
        Ld_B_HLptr(Ld<'a, lex::B<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_B_d8(Ld<'a, lex::B<'a>, lex::Lit<'a>>),
        // ld %c, <src>
        Ld_C_A(Ld<'a, lex::C<'a>, lex::A<'a>>),
        Ld_C_B(Ld<'a, lex::C<'a>, lex::B<'a>>),
        Ld_C_C(Ld<'a, lex::C<'a>, lex::C<'a>>),
        Ld_C_D(Ld<'a, lex::C<'a>, lex::D<'a>>),
        Ld_C_E(Ld<'a, lex::C<'a>, lex::E<'a>>),
        Ld_C_H(Ld<'a, lex::C<'a>, lex::L<'a>>),
        Ld_C_HLptr(Ld<'a, lex::C<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_C_d8(Ld<'a, lex::C<'a>, lex::Lit<'a>>),
        // ld %d, <src>
        Ld_D_A(Ld<'a, lex::D<'a>, lex::A<'a>>),
        Ld_D_B(Ld<'a, lex::D<'a>, lex::B<'a>>),
        Ld_D_C(Ld<'a, lex::D<'a>, lex::C<'a>>),
        Ld_D_D(Ld<'a, lex::D<'a>, lex::D<'a>>),
        Ld_D_E(Ld<'a, lex::D<'a>, lex::E<'a>>),
        Ld_D_H(Ld<'a, lex::D<'a>, lex::L<'a>>),
        Ld_D_HLptr(Ld<'a, lex::D<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_D_d8(Ld<'a, lex::D<'a>, lex::Lit<'a>>),
        // ld %e, <src>
        Ld_E_A(Ld<'a, lex::E<'a>, lex::A<'a>>),
        Ld_E_B(Ld<'a, lex::E<'a>, lex::B<'a>>),
        Ld_E_C(Ld<'a, lex::E<'a>, lex::C<'a>>),
        Ld_E_D(Ld<'a, lex::E<'a>, lex::D<'a>>),
        Ld_E_E(Ld<'a, lex::E<'a>, lex::E<'a>>),
        Ld_E_H(Ld<'a, lex::E<'a>, lex::L<'a>>),
        Ld_E_HLptr(Ld<'a, lex::E<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_E_d8(Ld<'a, lex::E<'a>, lex::Lit<'a>>),
        // ld %h, <src>
        Ld_H_A(Ld<'a, lex::H<'a>, lex::A<'a>>),
        Ld_H_B(Ld<'a, lex::H<'a>, lex::B<'a>>),
        Ld_H_C(Ld<'a, lex::H<'a>, lex::C<'a>>),
        Ld_H_D(Ld<'a, lex::H<'a>, lex::D<'a>>),
        Ld_H_E(Ld<'a, lex::H<'a>, lex::E<'a>>),
        Ld_H_H(Ld<'a, lex::H<'a>, lex::L<'a>>),
        Ld_H_HLptr(Ld<'a, lex::H<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_H_d8(Ld<'a, lex::H<'a>, lex::Lit<'a>>),
        // ld %l, <src>
        Ld_L_A(Ld<'a, lex::L<'a>, lex::A<'a>>),
        Ld_L_B(Ld<'a, lex::L<'a>, lex::B<'a>>),
        Ld_L_C(Ld<'a, lex::L<'a>, lex::C<'a>>),
        Ld_L_D(Ld<'a, lex::L<'a>, lex::D<'a>>),
        Ld_L_E(Ld<'a, lex::L<'a>, lex::E<'a>>),
        Ld_L_H(Ld<'a, lex::L<'a>, lex::L<'a>>),
        Ld_L_HLptr(Ld<'a, lex::L<'a>, Ptr<'a, lex::HL<'a>>>),
        Ld_L_d8(Ld<'a, lex::L<'a>, lex::Lit<'a>>),
        // ld (%hl), <src>
        Ld_HLptr_A(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::A<'a>>),
        Ld_HLptr_B(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::B<'a>>),
        Ld_HLptr_C(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::C<'a>>),
        Ld_HLptr_D(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::D<'a>>),
        Ld_HLptr_E(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::E<'a>>),
        Ld_HLptr_H(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::L<'a>>),

        Ld_HLptr_d8(Ld<'a, Ptr<'a, lex::HL<'a>>, lex::Lit<'a>>),
        Ld_HLptr_a16ptr(Ld<'a, Ptr<'a, lex::HL<'a>>, Ptr<'a, lex::Lit<'a>>>),
        // ld (%hl+), %a
        Ld_HLptr_inc_A(Ld<'a, PtrInc<'a, lex::HL<'a>>, lex::A<'a>>),
        // ld (%hl-), %a
        Ld_HLptr_dec_A(Ld<'a, PtrDec<'a, lex::HL<'a>>, lex::A<'a>>),
        // ld (%c), %a
        Ld_Cptr_A(Ld<'a, Ptr<'a, lex::C<'a>>, lex::A<'a>>),
        // ld %a, (%c)
        Ld_A_Cptr(Ld<'a, lex::A<'a>, Ptr<'a, lex::C<'a>>>),
        // ld (a16), %a
        Ld_a16ptr_A(Ld<'a, Ptr<'a, lex::Ident<'a>>, lex::A<'a>>),
        // inc <T>
        Inc_B(Inc<'a, lex::B<'a>>),
        Inc_D(Inc<'a, lex::D<'a>>),
        Inc_H(Inc<'a, lex::H<'a>>),
        Inc_HLptr(Inc<'a, Ptr<'a, lex::H<'a>>>),
        Inc_C(Inc<'a, lex::C<'a>>),
        Inc_E(Inc<'a, lex::E<'a>>),
        Inc_L(Inc<'a, lex::L<'a>>),
        Inc_A(Inc<'a, lex::A<'a>>),
        Inc_BC(Inc<'a, lex::BC<'a>>),
        Inc_DE(Inc<'a, lex::DE<'a>>),
        Inc_HL(Inc<'a, lex::HL<'a>>),
        Inc_SP(Inc<'a, lex::SP<'a>>),
        // .dec <T>
        Dec_B(Dec<'a, lex::B<'a>>),
        Dec_D(Dec<'a, lex::D<'a>>),
        Dec_H(Dec<'a, lex::H<'a>>),
        Dec_HLptr(Dec<'a, Ptr<'a, lex::H<'a>>>),
        Dec_C(Dec<'a, lex::C<'a>>),
        Dec_E(Dec<'a, lex::E<'a>>),
        Dec_L(Dec<'a, lex::L<'a>>),
        Dec_A(Dec<'a, lex::A<'a>>),
        Dec_BC(Dec<'a, lex::BC<'a>>),
        Dec_DE(Dec<'a, lex::DE<'a>>),
        Dec_HL(Dec<'a, lex::HL<'a>>),
        Dec_SP(Dec<'a, lex::SP<'a>>),
        // add %a, <T>
        Add_A_B(Add<'a, lex::A<'a>, lex::B<'a>>),
        Add_A_C(Add<'a, lex::A<'a>, lex::C<'a>>),
        Add_A_D(Add<'a, lex::A<'a>, lex::D<'a>>),
        Add_A_E(Add<'a, lex::A<'a>, lex::E<'a>>),
        Add_A_H(Add<'a, lex::A<'a>, lex::H<'a>>),
        Add_A_L(Add<'a, lex::A<'a>, lex::L<'a>>),
        Add_A_HLptr(Add<'a, lex::A<'a>, Ptr<'a, lex::HL<'a>>>),
        Add_A_A(Add<'a, lex::A<'a>, lex::A<'a>>),
        Add_A_d8(Add<'a, lex::A<'a>, lex::Lit<'a>>),
        // add %hl, <T>
        Add_HL_BC(Add<'a, lex::HL<'a>, lex::BC<'a>>),
        Add_HL_DE(Add<'a, lex::HL<'a>, lex::DE<'a>>),
        Add_HL_HL(Add<'a, lex::HL<'a>, lex::HL<'a>>),
        Add_HL_SP(Add<'a, lex::HL<'a>, lex::SP<'a>>),
        // add SP,r8
        Add_SP_r8(Add<'a, lex::SP<'a>, lex::Lit<'a>>),
        // sub %a, <T>
        Sub_B(Sub<'a, lex::B<'a>>),
        Sub_C(Sub<'a, lex::C<'a>>),
        Sub_D(Sub<'a, lex::D<'a>>),
        Sub_E(Sub<'a, lex::E<'a>>),
        Sub_H(Sub<'a, lex::H<'a>>),
        Sub_L(Sub<'a, lex::L<'a>>),
        Sub_HLptr(Sub<'a, Ptr<'a, lex::HL<'a>>>),
        Sub_A(Sub<'a, lex::A<'a>>),
        Sub_d8(Sub<'a, lex::Lit<'a>>),
        // and <T>
        And_B(And<'a, lex::B<'a>>),
        And_C(And<'a, lex::C<'a>>),
        And_D(And<'a, lex::D<'a>>),
        And_E(And<'a, lex::E<'a>>),
        And_H(And<'a, lex::H<'a>>),
        And_L(And<'a, lex::L<'a>>),
        And_HLptr(And<'a, Ptr<'a, lex::HL<'a>>>),
        And_A(And<'a, lex::A<'a>>),
        And_d8(And<'a, lex::Lit<'a>>),
        // xor <T>
        Xor_B(Xor<'a, lex::B<'a>>),
        Xor_C(Xor<'a, lex::C<'a>>),
        Xor_D(Xor<'a, lex::D<'a>>),
        Xor_E(Xor<'a, lex::E<'a>>),
        Xor_H(Xor<'a, lex::H<'a>>),
        Xor_L(Xor<'a, lex::L<'a>>),
        Xor_HLptr(Xor<'a, Ptr<'a, lex::HL<'a>>>),
        Xor_A(Xor<'a, lex::A<'a>>),
        Xor_d8(Xor<'a, lex::Lit<'a>>),
        // or <T>
        Or_B(Or<'a, lex::B<'a>>),
        Or_C(Or<'a, lex::C<'a>>),
        Or_D(Or<'a, lex::D<'a>>),
        Or_E(Or<'a, lex::E<'a>>),
        Or_H(Or<'a, lex::H<'a>>),
        Or_L(Or<'a, lex::L<'a>>),
        Or_HLptr(Or<'a, Ptr<'a, lex::HL<'a>>>),
        Or_A(Or<'a, lex::A<'a>>),
        Or_d8(Or<'a, lex::Lit<'a>>),
    }
}

impl<'a> Grammar<'a> for Option<Asm<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        unimplemented!()
    }
}

impl<'a> Grammar<'a> for Asm<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        unimplemented!()
    }
}

parse! {
    /// `<ident> :`
    pub struct Label<'a> {
        pub ident: lex::Ident<'a>,
        pub colon: lex::Colon<'a>,
    }
}

parse! {
    /// `.inc <T>`
    pub struct Inc<'a, T>
    where
        T: Grammar<'a>,
    {
        pub inc: lex::Inc<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.dec <T>`
    pub struct Dec<'a, T>
    where
        T: Grammar<'a>,
    {
        pub dec: lex::Dec<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.ld <L>, <R>`
    pub struct Ld<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub ldh: lex::Ldh<'a>,
        pub left: L,
        pub comma: lex::Comma<'a>,
        pub right: R,
    }
}

parse! {
    /// `.add <L>, <R>`
    pub struct Add<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub add: lex::Add<'a>,
        pub left: L,
        pub comma: lex::Comma<'a>,
        pub right: R,
    }
}

parse! {
    /// `.sub <T>`
    pub struct Sub<'a, T>
    where
        T: Grammar<'a>,
    {
        pub sub: lex::Sub<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.and <T>`
    pub struct And<'a, T>
    where
        T: Grammar<'a>,
    {
        pub and: lex::And<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.xor <T>`
    pub struct Xor<'a, T>
    where
        T: Grammar<'a>,
    {
        pub xor: lex::Xor<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.or <T>`
    pub struct Or<'a, T>
    where
        T: Grammar<'a>,
    {
        pub or: lex::Or<'a>,
        pub inner: T,
    }
}

parse! {
    /// `.ldh <L>, <R>`
    pub struct Ldh<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub ld: lex::Ld<'a>,
        pub left: L,
        pub comma: lex::Comma<'a>,
        pub right: R,
    }
}

parse! {
    /// `( <T> )`
    pub struct Ptr<'a, T>
    where
        T: Grammar<'a>,
    {
        pub left_par: lex::LeftPar<'a>,
        pub inner: T,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `( <T> + )`
    pub struct PtrInc<'a, T>
    where
        T: Grammar<'a>,
    {
        pub left_par: lex::LeftPar<'a>,
        pub inner: T,
        pub plus: lex::Plus<'a>,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `( <T> - )`
    pub struct PtrDec<'a, T>
    where
        T: Grammar<'a>,
    {
        pub left_par: lex::LeftPar<'a>,
        pub inner: T,
        pub minus: lex::Minus<'a>,
        pub right_par: lex::RightPar<'a>,
    }
}
