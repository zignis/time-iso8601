use ::time::format_description::well_known::{
    iso8601::{Config, EncodedConfig, TimePrecision},
    Iso8601,
};
use std::num::NonZeroU8;

const CONFIG: EncodedConfig = Config::DEFAULT
    .set_year_is_six_digits(false)
    .set_time_precision(TimePrecision::Second {
        decimal_digits: NonZeroU8::new(2),
    })
    .encode();

const FORMAT: Iso8601<CONFIG> = Iso8601::<CONFIG>;

time::serde::format_description!(time_format, OffsetDateTime, FORMAT);

pub use time_format::*;

pub mod option {
    pub use super::time_format::option::*;
}
