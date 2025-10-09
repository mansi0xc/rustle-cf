The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

Let’s pretend we’re the compiler. We’ll apply these rules to figure out the lifetimes of the references in the signature of the first_word function in Listing 10-25. The signature starts without any lifetimes associated with the references:

```bash
fn first_word(s: &str) -> &str {
```

Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime. We’ll call it 'a as usual, so now the signature is this:

```bash
fn first_word<'a>(s: &'a str) -> &str {
```
The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:

```bash
fn first_word<'a>(s: &'a str) -> &'a str {
```
Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

Let’s look at another example, this time using the longest function that had no lifetime parameters when we started working with it in Listing 10-20:

```bash
fn longest(x: &str, y: &str) -> &str {
```
Let’s apply the first rule: each parameter gets its own lifetime. This time we have two parameters instead of one, so we have two lifetimes:

```bash
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
You can see that the second rule doesn’t apply because there is more than one input lifetime. The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self. After working through all three rules, we still haven’t figured out what the return type’s lifetime is. This is why we got an error trying to compile the code in Listing 10-20: the compiler worked through the lifetime elision rules but still couldn’t figure out all the lifetimes of the references in the signature.

Because the third rule really only applies in method signatures, we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.