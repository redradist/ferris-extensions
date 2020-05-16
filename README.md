# FerrisExtensions

## BoxedAsyncRecursion
This macro `#[boxed_async_recursion]` is for helping recursion calls with async/.await

Previously for writing recursion with async/.await you should make the following trick:
```rust
fn recursive(k: u8, l: u32) -> BoxFuture<'static, u8> {
    async move {
        recursive(k, l).await;
        recursive(k, l).await;
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

## MultipleResultErrors

This macro `#[multiple_result_errors]` is for helping handle multiple errors from functions

Example:
```rust
#[multiple_result_errors]
fn handle_file() -> Result<(), (IOError, IOError2)>
{
    get_io_error()?;
    Ok(())
}
fn main() {
    let res = handle_file();
    match res {
        Ok(t) => {},
        Err(err) => {
            match err {
                HandleFileResultErrors::IOError(err0) => {},
                HandleFileResultErrors::IOError2(err1) => {},
            };
        }
    };
}
```

In this example with help of macros `#[multiple_result_errors]` will be generated anonymous enum `Handle22UrlResultErrors`
that simplify working with multiple errors
