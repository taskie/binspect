use binspect::binspect;

fn main() {
    let s = "ABC";
    binspect!(s);
    unsafe { binspect!(*s, s.len()) };
}
