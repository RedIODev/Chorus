public namespace Testfile;

public namespace MySpace {
    local struct Test(String s);

    const NUMBER:u32 = 5;

    public const inline unsafe fn mylocalfunc1<T>[a, b](Test t) -> String {
        t+= 5;
        var ptr = &mut t;   //comment test
        ptr.s = "Hello";
        return t.s;
    }
}

interface AddAssign<T = Self> {
    fn addAssign(&mut Self self, T other);
}

implement MySpace: AddAssign {
    fn addAssign(&mut Self self, Self other) {
        self.s += other.s
    }
}

struct MyTuple(i32, f32);

struct u40: unsigned 5; //primitive unsigned 40 bit struct