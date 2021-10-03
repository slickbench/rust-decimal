use juniper::{Value, ParseScalarResult, ParseScalarValue};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::Decimal;

#[juniper::graphql_scalar(description = "Decimal")]
impl<S> GraphQLScalar for Decimal
where
    S: ScalarValue + From<Decimal>
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(ToPrimitive::to_f64(self).unwrap_or_default())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<Decimal> {
        v.as_scalar_value()
        .and_then(|v| v.as_float().and_then(FromPrimitive::from_f64))
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}
