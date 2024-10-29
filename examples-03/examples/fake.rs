// use fake::{Dummy, Fake, Faker};
// use rand::rngs::StdRng;
// use rand::SeedableRng;
//
// #[derive(Debug, Dummy)]
// pub struct Foo {
//     #[dummy(faker = "1000..2000")]
//     order_id: usize,
//     customer: String,
//     paid: bool,
// }
//
// fn main() {
//
//     // type derived Dummy
//     let f: Foo = Faker.fake();
//     println!("{:?}", f);
//
//     // using `Faker` to generate default fake value of given type
//     let tuple = Faker.fake::<(u8, u32, f32)>();
//     println!("tuple {:?}", tuple);
//     println!("String {:?}", Faker.fake::<String>());
//
//     // types U can used to generate fake value T, if `T: Dummy<U>`
//     println!("String {:?}", (8..20).fake::<String>());
//     println!("u32 {:?}", (8..20).fake::<u32>());
//
//     // using `faker` module with locales
//     use fake::faker::name::raw::*;
//     use fake::locales::*;
//
//     let name: String = Name(EN).fake();
//     println!("name {:?}", name);
//
//     let name: String = Name(ZH_TW).fake();
//     println!("name {:?}", name);
//
//     // using convenient function without providing locale
//     use fake::faker::lorem::en::*;
//     let words: Vec<String> = Words(3..5).fake();
//     println!("words {:?}", words);
//
//     // Using a tuple config list to generate a vector with a length range and a specific faker for the element
//     let name_vec: Vec<String> = (Name(EN), 3..5).fake();
//
//     // Using a macro as an alternative method for the tuple config list
//     let name_vec = fake::vec![String as Name(EN); 3..5];
//
//     // using macro to generate nested collection
//     let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
//     println!("random nested vec {:?}", name_vec);
//
//     // fixed seed rng
//     let seed = [
//         1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0,
//     ];
//     let ref mut r = StdRng::from_seed(seed);
//     for _ in 0..5 {
//         let v: usize = Faker.fake_with_rng(r);
//         println!("value from fixed seed {}", v);
//     }
//
//     // Use an always true RNG so that optional types are always `Some` values. (Requires
//     // always-true-rng feature).
//     use fake::utils::AlwaysTrueRng;
//     let mut rng = AlwaysTrueRng::default();
//     let result: Option<i64> = Faker.fake_with_rng(&mut rng);
//     println!("Always Some: {}", result.unwrap());
// }
