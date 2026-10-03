trait Identifiable {
    fn id(&self) -> u32;
}
trait Debuggable {
    fn debug(&self);
}
struct KoelUser {
    id: u32,
}
struct KoelDevice {
    id: u32,
}
impl Identifiable for KoelUser {
    fn id(&self) -> u32 {
        self.id
    }
}
impl Identifiable for KoelDevice {
    fn id(&self) -> u32 {
        self.id
    }
}
//blanket implementation
// Any type that implements Identifiable automatically gets Debuggable.
impl<T: Identifiable> Debuggable for T {
    fn debug(&self) {
        println!("koel is : {}", self.id());
    }
}

fn main() {
    let user = KoelUser { id: 101 };
    let device = KoelDevice { id: 500 };

    user.debug();
    device.debug();
}
