public namespace MySpace {
    local struct Test(String s);

    const NUMBER:u32 = 5;

    public const inline unsafe fn mylocalfunc1<T>[a:heap, b:stack](Test t) -> String {
        t+= 5;
        var ptr = &mod t;
        ptr°s = "Hello";
        return t.s;
    }
}