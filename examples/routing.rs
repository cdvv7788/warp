#![deny(warnings)]
extern crate pretty_env_logger;
#[macro_use]
extern crate warp;

use warp::Filter;

fn main() {
    pretty_env_logger::init();

    // We'll start simple, and gradually show how you combine these powers
    // into super powers!

    // GET /hi
    let hi = warp::path("hi").map(|| {
        "Hello, World!"
    });

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = path!("hello" / "from" / "warp").map(|| {
        "Hello from warp!"
    });

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = path!("sum" / u32 / u32).map(|a, b| {
        format!("{} + {} = {}", a, b, a + b)
    });

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times = path!(u16 / "times" / u16).map(|a, b| {
        format!("{} times {} = {}", a, b, a * b)
    });

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye").and(warp::path::param()).map(|name: String| {
        format!("Good bye, {}!", name)
    });

    // Ah, can filters do things besides `and`?
    //
    // Why, yes they can! They can also `or`! As you might expect, `or` creates
    // a "this or else that" chain of filters. If the first doesn't succeed,
    // then it tries the other.
    //
    // So, those `math` routes could have been mounted all as one, with `or`.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math")
        .and(sum.or(times));

    // It turns out, using `or` is how you combine everything together into
    // a single API. (We also actually haven't been enforcing the that the
    // method is GET, so we'll do that too!)
    //
    // GET /hi
    // GET /hello/from/warp
    // GET /bye/:string
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16

    let routes = warp::get(
        hi
            .or(hello_from_warp)
            .or(bye)
            .or(math)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030));
}
