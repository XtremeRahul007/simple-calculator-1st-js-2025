use calculator_engine::*;

use crate::errors::InvalidOperation;

#[derive(Debug)]
pub enum UnaryOperation {
    SquareRoot,
    CubeRoot,
    Absolute,
    Factorial,
    Reciprocal,
    Sign,

    // Powers & Logarithms
    Exponential,
    PowerOfTen,
    NaturalLog,
    CommonLog,

    // Trigonometric
    Sine,
    Cosine,
    Tangent,
    Arcsine,
    Arccosine,
    Arctangent,

    // Hyperbolic
    HyperbolicSine,
    HyperbolicCosine,
    HyperbolicTangent,
    InverseHyperbolicSine,
    InverseHyperbolicCosine,
    InverseHyperbolicTangent,

    // Rounding & Formatting
    Truncate,
    Round,
    Ceiling,
    Floor,
    // Analysis
    /*PrimeFactors,*/
}

impl UnaryOperation {
    pub fn execute(
        self,
        number: &Number,
        precision: &PrecisionContext,
    ) -> Result<Number, CalculationError> {
        match self {
            Self::SquareRoot => square_root(number, precision),
            Self::CubeRoot => cube_root(number, precision),
            Self::Absolute => Ok(absolute(number)),
            Self::Factorial => {
                let n = number
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;

                factorial(n, precision)
            }
            Self::Reciprocal => reciprocal(number, precision),
            Self::Sign => sine(number, precision),
            Self::Exponential => exponential(number, precision),
            Self::PowerOfTen => power_of_ten(number, precision),
            Self::NaturalLog => natural_log(number, precision),
            Self::CommonLog => common_log(number, precision),
            Self::Sine => sine(number, precision),
            Self::Cosine => cosine(number, precision),
            Self::Tangent => tangent(number, precision),
            Self::Arcsine => arcsine(number, precision),
            Self::Arccosine => arccosine(number, precision),
            Self::Arctangent => arctangent(number, precision),
            Self::HyperbolicSine => hyperbolic_sine(number, precision),
            Self::HyperbolicCosine => hyperbolic_cosine(number, precision),
            Self::HyperbolicTangent => hyperbolic_tangent(number, precision),
            Self::InverseHyperbolicSine => inverse_hyperbolic_sine(number, precision),
            Self::InverseHyperbolicCosine => inverse_hyperbolic_cosine(number, precision),
            Self::InverseHyperbolicTangent => inverse_hyperbolic_tangent(number, precision),
            Self::Truncate => tangent(number, precision),
            Self::Round => Ok(round(number)),
            Self::Ceiling => Ok(round(number)),
            Self::Floor => Ok(floor(number)),
        }
    }
}

impl TryFrom<&str> for UnaryOperation {
    type Error = InvalidOperation;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "square_root" => Ok(Self::SquareRoot),
            "cube_root" => Ok(Self::CubeRoot),
            "absolute" => Ok(Self::Absolute),
            "factorial" => Ok(Self::Factorial),
            "reciprocal" => Ok(Self::Reciprocal),
            "sign" => Ok(Self::Sign),
            "exponential" => Ok(Self::Exponential),
            "power_of_ten" => Ok(Self::PowerOfTen),
            "natural_log" => Ok(Self::NaturalLog),
            "common_log" => Ok(Self::CommonLog),
            "sine" => Ok(Self::Sine),
            "cosine" => Ok(Self::Cosine),
            "tangent" => Ok(Self::Tangent),
            "arcsine" => Ok(Self::Arcsine),
            "arccosine" => Ok(Self::Arccosine),
            "arctangent" => Ok(Self::Arctangent),
            "hyperbolic_sine" => Ok(Self::HyperbolicSine),
            "hyperbolic_cosine" => Ok(Self::HyperbolicCosine),
            "hyperbolic_tangent" => Ok(Self::HyperbolicTangent),
            "inverse_hyperbolic_sine" => Ok(Self::InverseHyperbolicSine),
            "inverse_hyperbolic_cosine" => Ok(Self::InverseHyperbolicCosine),
            "inverse_hyperbolic_tangent" => Ok(Self::InverseHyperbolicTangent),
            "truncate" => Ok(Self::Truncate),
            "round" => Ok(Self::Round),
            "ceiling" => Ok(Self::Ceiling),
            "floor" => Ok(Self::Floor),
            _ => Err(InvalidOperation(value.to_string())),
        }
    }
}
