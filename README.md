# uni_log
An ergonomic, minimalistic logger designed for what you need most! 

## NOTE
This is the newest, updated version of [mini_log](https://crates.io/crates/mini_log) \
mini_log has since been deprecated, and will no longer be updated. 

## How to use
1. First, include uni_log in your Cargo.toml
```
uni_log = "*"
```

2. Use uni_log in your code
``` 
use uni_log::*;
```

3. Call any of the Macro's anywhere
```
mark!("Mark");
info!("Info", 1);
warn!("Warning", 2);
error!("Error!" -1);
parse_log!();
```