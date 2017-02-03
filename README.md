# derivation
Deriving traits for C-like enums using macro 1.1.

This crate make 2 traits derivable for C-like enums, namely `Variants` and `FromStr`.
The first one could be derived for enums to transform it into a list of variants, and the second make parsing from `&str` to variants easy as a piece of cake.

Supports Rust 1.15 and above.

How to use:

1. Add these 2 crate as dependencies in your `Cargo.toml`

   ```toml
   [dependencies]
   
   derivation = "^0.1"
   enum_variants = "^1.0"
   ```

2. Now import this crate in your source code at the top of your crate

   ```rust
   #[macro_use]
   extern crate derivation;
   ```
   
3. The trait `Variants` has only one method: `fn variants() -> Vec<Self>;` which should return the list of all variants.
   Import it when you need to derive it:

   ```rust
   extern crate enum_variants;
   
   use enum_variants::Variants;
   ```

4. Derive `FromStr` or `Variants` like you do with other traits:
   
   ```rust
   #[derive(FromStr, Variants)]
   enum Enum {
       /* ... */
   }
   ```

Notice that only C-like (unitary) enums are supported!
