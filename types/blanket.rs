trait Identifiable {
    fn id(&self) -> u32;
}

//blanket implementation
// Every type that implements Identifiable automatically gets Debuggable.

trait Debuggable {
    fn debug(&self);
}

impl<T: Identifiable> Debuggable for T {
    fn debug(&self) {
        println!("koel is : {}", self.id());
    }
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
fn main() {
    let user = KoelUser { id: 101 };
    let device = KoelDevice { id: 500 };

    user.debug();
    device.debug();
}
