use crate::collections::base::span_set::SpanSet;

/// You shouldn't probably implement this trait yourself, it's just to run some functions in both IntSpanSet and FloatSpanSet
pub trait NumberSpanSet: SpanSet {}
