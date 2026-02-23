use time::OffsetDateTime;

pub mod blog {
    tonic::include_proto!("blog");
}

pub fn parse_proto_timestamp(value: Option<OffsetDateTime>) -> Option<pbjson_types::Timestamp> {
    value.map(|v| pbjson_types::Timestamp {
        seconds: v.unix_timestamp(),
        nanos: v.nanosecond() as i32,
    })
}
