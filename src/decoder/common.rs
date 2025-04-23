use nom::{IResult, Parser, bytes::complete::tag, combinator::value};

pub fn byte(input: &[u8]) -> IResult<&[u8], u8> {
    nom::number::u8().parse(input)
}

pub fn packed_byte(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}

macro_rules! constant {
    ($construct:ident = $byte:literal) => {
        #[derive(Debug, Clone)]
        pub struct $construct;

        impl crate::decoder::common::GifConstant for $construct {
            const BYTE: u8 = $byte;
            fn make() -> Self {
                Self
            }
        }

        impl<'a> nom::Parser<&'a [u8]> for $construct {
            type Output = Self;
            type Error = nom::error::Error<&'a [u8]>;

            fn process<OM: nom::OutputMode>(
                &mut self,
                input: &'a [u8],
            ) -> nom::PResult<OM, &'a [u8], Self::Output, Self::Error> {
                <Self as crate::decoder::common::GifConstant>::parser().process::<OM>(input)
            }
        }
    };
}

pub(super) use constant;

pub trait GifConstant: Sized + Clone {
    const BYTE: u8;
    fn make() -> Self;

    fn parser<'a>() -> impl Parser<&'a [u8], Output = Self, Error = nom::error::Error<&'a [u8]>> {
        value(Self::make(), tag([Self::BYTE].as_ref()))
    }
}
