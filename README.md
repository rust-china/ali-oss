# AliOss &emsp;

[![ci](https://github.com/rust-china/ali-oss/workflows/Rust/badge.svg)](https://github.com/rust-china/ali-oss/actions)
[![Latest Version]][crates.io]
![downloads](https://img.shields.io/crates/d/ali-oss.svg?style=flat-square)

[Latest Version]: https://img.shields.io/crates/v/ali-oss.svg
[crates.io]: https://crates.io/crates/ali-oss

### Usage

```rust
let oss_client = crate::Client::from_env()?;
let buckets = oss_client.list_buckets().await?;
println!("buckets: {:?}", buckets);
```

env config

```
ALI_OSS_ACCESS_KEY_ID=xxx
ALI_OSS_ACCESS_KEY_SECRET=xxx
ALI_OSS_BUCKET=xxx
ALI_OSS_LOCATION=oss-cn-hangzhou
ALI_OSS_PATH=/
ALI_OSS_INTERNAL=false

```

### Methods:

- list_buckets()
- put_bucket()
- get_bucket_info()
- get_bucket_location()
- get_bucket_stat()
- delete_bucket()

- list_objects(prefix, delimiter)
- put_object(object_name, byptes)
- put_object_stream(object_name, stream)
- get_object(object_name)
- delete_object(object_name)
- delete_multiple_objects(object_names)
- copy_object(dest_object_name, source_object_name)
- append_object(object_name, byptes, position)
- head_object(object_name)
- get_object_meta(object_name)
- is_object_exist(object_name)

- sign_object(object_name, duration_time)

- put_symlink(symlink_object_name, target_object_name)
- get_symlink(symlink_object_name)
