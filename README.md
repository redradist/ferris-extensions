# AsyncHelpers
This is test cargo crate for helping recursion calls with async/.await

Previosly for writing recursion with async/.await you should make the following trick:
```rust
fn recursive() -> BoxFuture<'static, u8> {
    async move {
        recursive().await;
        recursive().await;
        2u8
    }.boxed()
}
```

With this crate it is possible to simplify code as follows:
```rust
#[boxed_async_recursion]
async fn recursive(k: u8, l: u32) -> u8 {
    recursive(k, l).await;
    recursive(k, l).await;
    2u8
}
```

Under the hood compiler macros still generates BoxFuture and async move, but it much easy to understand code right know
