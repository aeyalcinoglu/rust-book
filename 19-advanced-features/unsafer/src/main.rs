use std::slice;

fn _safe_but_bad(address: usize) -> &'static mut [i32] {
    let r = address as *mut i32;

    unsafe { slice::from_raw_parts_mut(r, 10000) }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut YEAR: i32 = 2022;

unsafe trait Foo {}
unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let _r = address as *const i32;
    unsafe {
        println!("{}, {}", *r1, *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;
    // use r.split_at_mut(3) for std
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // println!("{:?}", _safe_but_bad(address));

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    unsafe {
        YEAR += 1;
        println!("We live in {}", YEAR);
    }
}
