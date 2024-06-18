# AliOss &emsp;

[![ci](https://github.com/rust-china/ali-oss/workflows/Rust/badge.svg)](https://github.com/rust-china/ali-oss/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/ali-oss.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/ali-oss.svg
[crates.io]: https://crates.io/crates/ali-oss

### Methods:

- list_buckets()
- put_bucket()
- get_bucket_info()
- get_bucket_location()
- get_bucket_stat()
- delete_bucket()

- list_objects()
- put_object(object_name, byptes)
- put_object_stream(object_name, stream)
- get_object(object_name)
- delete_object(object_name)
- copy_object(from_object, to_object_name)
- append_object(object_name, byptes, position)
- head_object(object_name)
- get_object_meta(object_name)
