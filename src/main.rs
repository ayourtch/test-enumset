#[macro_use]
extern crate enumset;
use enumset::EnumSet;
use std::convert::TryFrom;

#[derive(EnumSetType, Debug)]
#[repr(u32)]
enum FooTest {
    OneValue = 2,
    TwoValue = 4,
    BigValue = 0x8000000,
}

const CONST_SET: EnumSet<FooTest> = enum_set!(FooTest::OneValue | FooTest::TwoValue);

fn main() {
    let v: EnumSet<FooTest> = FooTest::OneValue | FooTest::TwoValue | FooTest ::BigValue;
    println!("Hello, world!, {:?}", &v);
    if CONST_SET == v {
        println!("Sets are equal!");
    }
    println!("Size of : {}", std::mem::size_of::<EnumSet<FooTest>>());

    println!("v: {:x}", v.as_u32() );
}
