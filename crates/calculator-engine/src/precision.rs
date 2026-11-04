use dashu::float::{Context as FloatContext, round::mode::HalfAway};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrecisionContext {
    significant_digits: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrecisionError {
    ZeroPrecision,
}

impl PrecisionContext {
    pub fn new(significant_digits: usize) -> Result<Self, PrecisionError> {
        if significant_digits == 0 {
            return Err(PrecisionError::ZeroPrecision);
        }

        Ok(Self { significant_digits })
    }

    pub const fn precision(&self) -> usize {
        self.significant_digits
    }

    pub fn float_context(&self) -> FloatContext<HalfAway> {
        FloatContext::new(self.significant_digits)
    }
}

impl Default for PrecisionContext {
    fn default() -> Self {
        Self {
            significant_digits: 50,
        }
    }
}
