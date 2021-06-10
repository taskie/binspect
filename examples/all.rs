#![allow(dead_code)]
use binspect::binspect;
use std::mem;

fn main() {
    println!("# Examples");
    println!();
    println!("This document was generated by `cargo run --example all >examples.md`.");
    println!();
    section("Primitives", PRIMITIVES, primitives);
    section("Arrays", ARRAYS, arrays);
    section("Tuples", TUPLES, tuples);
    section("Structs", STRUCTS, structs);
    section("Packed Structs", PACKED_STRUCTS, packed_structs);
    section("C Structs", C_STRUCTS, c_structs);
    section("Enums", ENUMS, enums);
    section("Slices", SLICES, slices);
    section("Vecs", VECS, vecs);
    section("Strs", STRS, strs);
    section("Strings", STRINGS, strings);
    section("Trait Objects", TRAIT_OBJECTS, trait_objects);
    section("Boxes", BOXES, boxes);
    section("Options", OPTIONS, options);
}

macro_rules! code_fence {
    ($name:ident, { $($all:item)+ }) => {
        const $name: &'static str = stringify!($($all)+);
        $($all)+
    };
}

code_fence!(PRIMITIVES, {
    fn primitives() {
        binspect!(false);
        binspect!(true);
        binspect!(42);
        binspect!(-42);
        binspect!(127u8);
        binspect!(0xabcd_u16);
        binspect!(0xefbeadde_u32);
        binspect!(5000_0000_0000_0000_u64);
        binspect!('A');
        binspect!('あ');
        binspect!('😇');
        binspect!(0.25f32);
        binspect!(0.1);
    }
});

code_fence!(ARRAYS, {
    fn arrays() {
        binspect!([true, false, false, true]);
        binspect!([0xadde_u16, 0xefbe]);
        binspect!(b"\xde\xad\xbe\xef");
        binspect!(*b"\xde\xad\xbe\xef");
        binspect!([0_u64; 0]);
        binspect!([[1_i8, 2, 3], [4, 5, 6], [7, 8, 9]]);
    }
});

code_fence!(TUPLES, {
    fn tuples() {
        binspect!(());
        binspect!((0xefbeadde_u32));
        binspect!((0x1111111111111111_u64, 0x22222222_u32, 0x_3333u16, 0x44_u8));
        binspect!((0x11_u8, 0x2222_u16, 0x33_u8));
        binspect!(((), (0xefbeadde_u32), (0x11_u8, 0x2222_u16, 0x33_u8)));
    }
});

code_fence!(STRUCTS, {
    struct S1;
    struct S2 {
        x: u32,
    }
    struct S3 {
        x: u64,
        y: u32,
        z: u16,
        w: u8,
    }
    struct S4 {
        x: u8,
        y: u16,
        z: u8,
    }
    struct S5 {
        s1: S1,
        s2: S2,
        s4: S4,
    }

    fn structs() {
        binspect!(S1 {});
        binspect!(S2 { x: 0xefbeadde_u32 });
        binspect!(S3 {
            x: 0x1111111111111111_u64,
            y: 0x22222222_u32,
            z: 0x_3333u16,
            w: 0x44_u8
        });
        binspect!(S4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8
        });
        binspect!(S5 {
            s1: S1 {},
            s2: S2 { x: 0xefbeadde_u32 },
            s4: S4 {
                x: 0x11_u8,
                y: 0x2222_u16,
                z: 0x33_u8
            }
        });
    }
});

code_fence!(PACKED_STRUCTS, {
    #[repr(packed)]
    struct PS1;
    #[repr(packed)]
    struct PS2 {
        x: u32,
    }
    #[repr(packed)]
    struct PS3 {
        x: u64,
        y: u32,
        z: u16,
        w: u8,
    }
    #[repr(packed)]
    struct PS4 {
        x: u8,
        y: u16,
        z: u8,
    }
    #[repr(packed)]
    struct PS5 {
        s1: PS1,
        s2: PS2,
        s4: PS4,
    }

    fn packed_structs() {
        binspect!(PS1 {});
        binspect!(PS2 { x: 0xefbeadde_u32 });
        binspect!(PS3 {
            x: 0x1111111111111111_u64,
            y: 0x22222222_u32,
            z: 0x_3333u16,
            w: 0x44_u8
        });
        binspect!(PS4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8
        });
        binspect!(PS5 {
            s1: PS1 {},
            s2: PS2 { x: 0xefbeadde_u32 },
            s4: PS4 {
                x: 0x11_u8,
                y: 0x2222_u16,
                z: 0x33_u8
            }
        });
    }
});

code_fence!(C_STRUCTS, {
    #[repr(C)]
    struct CS1;
    #[repr(C)]
    struct CS2 {
        x: u32,
    }
    #[repr(C)]
    struct CS3 {
        x: u64,
        y: u32,
        z: u16,
        w: u8,
    }
    #[repr(C)]
    struct CS4 {
        x: u8,
        y: u16,
        z: u8,
    }
    #[repr(C)]
    struct CS5 {
        s1: CS1,
        s2: CS2,
        s4: CS4,
    }

    fn c_structs() {
        binspect!(CS1 {});
        binspect!(CS2 { x: 0xefbeadde_u32 });
        binspect!(CS3 {
            x: 0x1111111111111111_u64,
            y: 0x22222222_u32,
            z: 0x_3333u16,
            w: 0x44_u8
        });
        binspect!(CS4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8
        });
        binspect!(CS5 {
            s1: CS1 {},
            s2: CS2 { x: 0xefbeadde_u32 },
            s4: CS4 {
                x: 0x11_u8,
                y: 0x2222_u16,
                z: 0x33_u8
            }
        });
    }
});

code_fence!(ENUMS, {
    enum E1 {
        V1,
        V2,
        V3,
    }
    enum E2 {
        V1(u32),
        V2(u8, u16, u8),
        V3,
    }

    fn enums() {
        binspect!(E1::V1);
        binspect!(E1::V2);
        binspect!(E1::V3);
        binspect!(E2::V1(0xefbeadde));
        binspect!(E2::V2(0x11, 0x2222, 0x33));
        binspect!(E2::V3);
    }
});
code_fence!(SLICES, {
    fn slices() {
        let bs = &[0xadde_u16, 0xefbe];
        binspect!(bs);
        unsafe { binspect!(*bs, mem::size_of::<u16>() * bs.len()) };
    }
});

code_fence!(VECS, {
    fn vecs() {
        let bs = vec![0xadde_u16, 0xefbe];
        binspect!(bs);
        binspect!(bs.as_ref() as &[u16]);
        unsafe { binspect!(*bs, mem::size_of::<u16>() * bs.len()) };
    }
});

code_fence!(STRS, {
    fn strs() {
        let s = "Hello, world!";
        binspect!(s);
        unsafe { binspect!(*s, s.len()) };
        let s = "あ";
        binspect!(s);
        unsafe { binspect!(*s, s.len()) };
        let s = "😇";
        binspect!(s);
        unsafe { binspect!(*s, s.len()) };
    }
});

code_fence!(STRINGS, {
    fn strings() {
        let s = "Hello, world!".to_owned();
        binspect!(s);
        binspect!(s.as_str());
        unsafe { binspect!(*s, s.len()) };
        let s = "あ".to_owned();
        binspect!(s);
        binspect!(s.as_str());
        unsafe { binspect!(*s, s.len()) };
        let s = "😇".to_owned();
        binspect!(s);
        binspect!(s.as_str());
        unsafe { binspect!(*s, s.len()) };
    }
});

code_fence!(TRAIT_OBJECTS, {
    trait T1 {
        fn m1(&self) {
            println!("trait T1");
        }
    }

    impl T1 for S4 {
        fn m1(&self) {
            println!("impl T1 for S4");
        }
    }

    fn trait_objects() {
        let t: &dyn T1 = &S4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8,
        };
        binspect!(t);
        unsafe { binspect!(*t, mem::size_of::<S4>()) };
    }
});

code_fence!(BOXES, {
    fn boxes() {
        let s = Box::new(S4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8,
        });
        binspect!(s);
        let p = Box::into_raw(s);
        unsafe { binspect!(*p) };
        unsafe { Box::from_raw(p) };
        let t: Box<dyn T1> = Box::new(S4 {
            x: 0x11_u8,
            y: 0x2222_u16,
            z: 0x33_u8,
        });
        binspect!(t);
        let p = Box::into_raw(t);
        unsafe { binspect!(*p, mem::size_of::<S4>()) };
        unsafe { Box::from_raw(p) };
    }
});

code_fence!(OPTIONS, {
    fn options() {
        binspect!(Some(255));
        binspect!(None as Option<i32>);
        let s2 = S2 { x: 255 };
        binspect!(Some(&s2));
        binspect!(None as Option<&S2>);
        binspect!(Some(&s2 as *const _));
        binspect!(None as Option<*const S2>);
    }
});

fn section<F: Fn()>(name: &str, code: &str, f: F) {
    println!("## {}", name);
    println!();
    println!("```rust");
    println!("{}", code);
    println!("```");
    println!();
    println!("```text");
    f();
    println!("```");
    println!();
}
