public namespace MySpace {
    local struct Test(String s);

    fn mylocalfunc1(Test t) -> String {
        t+= 5;
        return t.s;
    }
}