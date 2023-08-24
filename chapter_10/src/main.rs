mod generics;
mod traits;
mod trait_bound;
mod traits_return;
mod condi_trait;
mod lifetime;
mod lifetime1;
mod lifetime3;
mod lifetime2;
fn main() {
    generics::generics_use();
    traits :: traits_use();
    condi_trait::use_me();
    lifetime::left_time();
    lifetime1::left_time();
    lifetime2::use_me();
    lifetime3 :: usecase();
}
