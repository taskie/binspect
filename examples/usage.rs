use binspect::binspect;

fn main() {
    let s = "ABC";
    binspect!(s);
    binspect!(*s);
}
