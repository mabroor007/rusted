// Rust docs are amazing. No setup required and battries included

// let's say sum is a function i want to document
// all i have to do is to put /// on top of that
// function and start writing documentation.
// Bonus! It also supports markdown as well

/// adds two numbers
///
/// # Arguments
///
/// * `a` - first number
/// * `b` - second number
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// My-take
//   i think builtin docs are really really great for large projects
//   and we must doc our code properly
