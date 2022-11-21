# Qog

An extremely simple log library

## Quick Start

```rust
// Qog::new(log::Level::Debug, "demo.log".to_string()).init();
Qog::default();

log::trace!("23333, {}", 234);
log::debug!("23333, {}", 234);
log::info!("23333, {}", 234);
log::warn!("23333, {}", 234);
log::error!("23333, {}", 234);
```
