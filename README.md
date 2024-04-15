# time-iso8601

Serializes and deserializes [`OffsetDateTime`](https://docs.rs/time/latest/time/struct.OffsetDateTime.html) from the
[`time`](https://docs.rs/time) crate in 4-digit year format with 2-digit time precision to
work with the majority of web frameworks.

# Examples

```rust
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(with = "time-iso8601")]
    datetime_field: OffsetDateTime,
    #[serde(with = "time-iso8601::option")]
    optional_field: Option<OffsetDateTime>,
}
```
