# derivation
Deriving traits for C-like enums using macro 1.1.

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
   
3. The trait `Variants` has only one method: `fn variants() -> Vec<Self>;`.
   Import it when you need to derive it:

   ```rust
   extern crate enum_variants;
   
   use enum_variants::Variants;
   ```

Notice that only C-like (unitary) enums are supported!
