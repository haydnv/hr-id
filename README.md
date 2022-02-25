# hr-id

A human-readable unique ID for Rust which:
 - supports Unicode
 - disallows whitespace
 - disallows common control characters `..` and each of ``/~$`&|=^{}<>'\?:@#()``
 - disallows ASCII control characters (bytes with integer value < 32)
 - can be used to represent a domain name or URI segment but not an entire URL
 - implements `From<Uuid>`
 - implements `Serialize` and `Deserialize` with the `serde` feature
 - implements `ToStream` and `FromStream` with the `destream` feature
