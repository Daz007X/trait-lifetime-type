use std::fmt::Debug;

trait Container {
    type Ref<'a,T>
    where
        T :Debug +'a,
        Self : 'a;

    fn get_ref<'g,G>(&self, value: &'g G) -> Self::Ref<'g,G>
    where
        G:Debug + 'g
    ;
    
}

struct Holder;

impl Container for Holder {
    type Ref<'r0,R0> = &'r0 R0
    where
        R0:Debug +'r0,
    ;
    
    fn get_ref<'r1,R1>(&self, value: &'r1 R1) -> Self::Ref<'r1,R1>
    where
        R1: Debug + 'r1
    {
        value
    }
}

fn main() {
    let holder = Holder;
    let num = "42";

    let result = holder.get_ref(&num);

    println!("Result: {:?}", result);
}