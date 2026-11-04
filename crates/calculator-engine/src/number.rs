use dashu::{Decimal, Integer, Rational};

#[derive(Debug, PartialEq)]
pub enum Number {
    Exact(Rational),
    Approximate(Decimal),
}

impl Number {
    pub fn from_integer<T: Into<Integer>>(value: T) -> Self {
        Self::Exact(Rational::from(value.into()))
    }

    pub fn from_rational(value: Rational) -> Self {
        Self::Exact(value)
    }

    pub fn from_approximate(value: Decimal) -> Self {
        Self::Approximate(value)
    }

    pub fn parse_exact(input: &str) -> Result<Self, dashu::base::ParseError> {
        let input = input.trim();

        let value = if input.contains("/") {
            input.parse::<Rational>()?
        } else {
            Rational::from_str_decimal(input)?
        };

        Ok(Self::Exact(value))
    }

    pub fn parse_approximate(input: &str) -> Result<Self, dashu::base::ParseError> {
        Ok(Self::Approximate(input.trim().parse()?))
    }
}
