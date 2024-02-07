public namespace Testfile;

public namespace MySpace {
    local struct Test(String s);

    const NUMBER:u32 = 5;

    public const inline unsafe fn mylocalfunc1<T>(t: Test) -> String {
        t+= 5;
        var ptr = &mut t;   //comment test
        ptr.s = "Hello";
        return t.s;
    }
}

interface AddAssign<T = Self> {
    fn addAssign(self: &mut Self, other: T);
}

implement MySpace::Test: AddAssign {
    fn addAssign(self: &mut Self, other: T) {
        self.s += other.s
    }
}

struct MyTuple(i32, f32);

struct u40: unsigned 5; //primitive unsigned 40 bit struct

fn forLoop(iter:Iterable) {
    for i in iter {
        
    }
}

fn func[x,y]<A,B>(a:A,b:B) -> C {
    
}

interface Alloc<P = (), R = Self>
where P is Tuple {
    fn alloc(params: P) -> R {
        ...
    }
}