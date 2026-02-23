// use time::OffsetDateTime;

// use crate::domain;

// pub mod blog {
//     tonic::include_proto!("blog");
// }

// pub fn parse_proto_timestamp(value: Option<OffsetDateTime>) -> Option<pbjson_types::Timestamp> {
//     value.map(|v| pbjson_types::Timestamp {
//         seconds: v.unix_timestamp(),
//         nanos: v.nanosecond() as i32,
//     })
// }

// impl From<domain::post::Post> for blog::Post {
//     fn from(post: domain::post::Post) -> Self {
//         blog::Post {
//             id: post.id.to_string(),
//             author_id: post.author_id.to_string(),
//             title: post.title,
//             content: post.content,
//             created_at: parse_proto_timestamp(post.created_at),
//         }
//     }
// }
