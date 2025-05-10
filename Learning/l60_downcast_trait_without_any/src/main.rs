// l60_downcast_trait_without_any
// How to downcast a trait using a type_id discriminator
//
// 2025-05-10   PV

trait MyTrait {
    fn type_id(&self) -> &'static str;
}

struct MyType1<'a> {
    data: &'a str,
}

impl<'a> MyTrait for MyType1<'a> {
    fn type_id(&self) -> &'static str {
        "MyType1"
    }
}

struct MyType2<'a> {
    donnees: &'a str,
}

impl<'a> MyTrait for MyType2<'a> {
    fn type_id(&self) -> &'static str {
        "MyType2"
    }
}

fn downcast_if_my_type1<'a>(n: &'a dyn MyTrait) -> Option<&'a MyType1<'a>> {
    if n.type_id() == "MyType1" {
        // SAFETY: We have checked the type_id, so we believe this cast is valid.
        // However, this relies on the correctness of the type_id implementation.
        Some(unsafe { &*(n as *const dyn MyTrait as *const MyType1<'a>) })
    } else {
        None
    }
}

fn downcast_if_my_type2<'a>(n: &'a dyn MyTrait) -> Option<&'a MyType2<'a>> {
    if n.type_id() == "MyType2" {
        // SAFETY: We have checked the type_id, so we believe this cast is valid.
        // However, this relies on the correctness of the type_id implementation.
        Some(unsafe { &*(n as *const dyn MyTrait as *const MyType2<'a>) })
    } else {
        None
    }
}

fn main() {
    let data = String::from("hello");
    let my_instance = MyType1 { data: &data };
    let trait_object: &dyn MyTrait = &my_instance;

    if let Some(my_type1_ref) = downcast_if_my_type1(trait_object) {
        println!("Successfully downcasted to MyType1 with data: {}", my_type1_ref.data);
    }
    else if let Some(my_type2_ref) = downcast_if_my_type2(trait_object) {
        println!("Successfully downcasted to MyType2 with donnees: {}", my_type2_ref.donnees);
    } else {
        println!("Failed to downcast to MyType.");
    }
}