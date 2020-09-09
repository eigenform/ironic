
use ironic::mem::host::HostMemBacking;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

#[derive(Eq, PartialEq, Hash)]
enum RegionId { Foo, Bar, Baz }


macro_rules! write { ($kind:ty, $addr:expr, $val:expr) => { 
    unsafe { std::ptr::write_volatile::<$kind>($addr as *mut $kind, $val) };
}}
macro_rules! read { ($kind:ty, $addr:expr) => { 
    unsafe { std::ptr::read_volatile::<$kind>($addr as *const $kind) };
}}



#[test]
fn region_split() {
    let mut back = HostMemBacking::<RegionId>::new("split", 0x0001_0000);

    back.add_region(RegionId::Foo, 0x0000_0000, 0x1000_0000, 0x0000_8000);
    back.add_region(RegionId::Bar, 0x0000_8000, 0x1001_0000, 0x0000_8000);

    write!(u32, 0x1000_0000, 0xdeadcafe);
    write!(u32, 0x1001_0000, 0xdeadbeef);

    assert_eq!(0xdeadcafe, read!(u32, 0x1000_0000));
    assert_eq!(0xdeadbeef, read!(u32, 0x1001_0000));
}

#[test]
fn region_persist() {
    let mut back = HostMemBacking::<RegionId>::new("persist", 0x0001_0000);

    back.add_region(RegionId::Foo, 0x0000_0000, 0x2000_0000, 0x0000_8000);
    back.add_region(RegionId::Bar, 0x0000_8000, 0x2001_0000, 0x0000_8000);

    write!(u32, 0x2000_0000, 0xdeadcafe);
    write!(u32, 0x2001_0000, 0xdeadbeef);

    assert_eq!(0xdeadcafe, read!(u32, 0x2000_0000));
    assert_eq!(0xdeadbeef, read!(u32, 0x2001_0000));

    back.disable_region(RegionId::Bar);
    back.enable_region(RegionId::Bar, 0x0000_8000, 0x2001_0000);

    // Expect that the data is still resident
    assert_eq!(0xdeadbeef, read!(u32, 0x2001_0000));
}

#[test]
fn region_boundary() {
    let mut back = HostMemBacking::<RegionId>::new("boundary", 0x0001_0000);

    back.add_region(RegionId::Foo, 0x0000_0000, 0x3000_0000, 0x0000_8000);
    back.add_region(RegionId::Bar, 0x0000_8000, 0x3000_8000, 0x0000_8000);

    write!(u64, 0x3000_7ffc, 0xdeadcafe_deadcafe);

    assert_eq!(0xdeadcafe, read!(u32, 0x3000_7ffc));
    assert_eq!(0xdeadcafe, read!(u32, 0x3000_8000));
}

#[test]
fn region_mirror() {
    let mut back = HostMemBacking::<RegionId>::new("mirror", 0x0001_0000);

    back.add_region(RegionId::Foo, 0x0000_0000, 0x4000_0000, 0x0001_0000);
    back.add_region(RegionId::Bar, 0x0000_0000, 0x4001_0000, 0x0001_0000);
    back.add_region(RegionId::Baz, 0x0000_0000, 0x4002_0000, 0x0001_0000);

    write!(u32, 0x4000_0000, 0xdeadcafe);
    write!(u32, 0x4002_2000, 0xdeadbeef);

    assert_eq!(0xdeadcafe, read!(u32, 0x4000_0000));
    assert_eq!(0xdeadcafe, read!(u32, 0x4001_0000));
    assert_eq!(0xdeadcafe, read!(u32, 0x4002_0000));

    assert_eq!(0xdeadbeef, read!(u32, 0x4000_2000));
    assert_eq!(0xdeadbeef, read!(u32, 0x4001_2000));
    assert_eq!(0xdeadbeef, read!(u32, 0x4002_2000));

}


