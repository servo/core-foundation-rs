use std;

type id = libc::intptr_t;
type Class = libc::intptr_t;
type IMP = *u8;
type SEL = libc::intptr_t;
type Ivar = libc::intptr_t;

const nil : id = 0 as id;

native mod objc {
    fn class_addMethod(cls : Class,
                       name : SEL,
                       imp : IMP,
                       types : *libc::c_char) -> bool;
    fn class_addIvar(cls : Class,
                     name : *libc::c_char,
                     size : libc::size_t,
                     alignment: u8,
		     types: *libc::c_char) -> bool;
    fn object_setInstanceVariable(obj : id,
                                  name : *libc::c_char,
                                  value : *libc::c_void);
    fn object_getInstanceVariable(obj : id,
                                  name : *libc::c_char,
                                  outValue : **libc::c_void);
    fn objc_allocateClassPair(superclass : Class,
                              name : *libc::c_char,
                              extraBytes : libc::size_t) -> Class;
    fn objc_getClass(name : *libc::c_char) -> id;
    fn objc_msgSend(theReceiver : id, theSelector : SEL) -> id;
    fn objc_registerClassPair(cls : Class);
    fn sel_registerName(name : *libc::c_char) -> SEL;
}

#[test]
fn test_nsapp() {
    let klass = str::as_c_str("NSApplication") { |s|
        objc::objc_getClass(s)
    };

    let sel = str::as_c_str("sharedApplication") { |s|
        objc::sel_registerName(s)
    };

    let nsapp = objc::objc_msgSend(klass, sel);

    io::println(#fmt("nsapp: %d", (nsapp as int)));
}

#[test]
fn test_custom_obj() {
    crust fn MyObject_doSomething(this : id, _sel : SEL) -> id {
        io::println("doSomething");
        ret this;
    }

    let NSObject = str::as_c_str("NSObject") { |s|
        objc::objc_getClass(s)
    };
    let MyObject = str::as_c_str("MyObject") { |s|
        objc::objc_allocateClassPair(NSObject, s, 0 as libc::size_t)
    };
    let doSomething = str::as_c_str("doSomething") { |s|
        objc::sel_registerName(s)
    };
    let _ = str::as_c_str("@@:") { |types|
        objc::class_addMethod(MyObject, doSomething, MyObject_doSomething,
                              types)
    };
    objc::objc_registerClassPair(MyObject);

    let alloc = str::as_c_str("alloc") { |s| objc::sel_registerName(s) };
    let init = str::as_c_str("init") { |s| objc::sel_registerName(s) };

    let mut obj = objc::objc_msgSend(MyObject, alloc);
    obj = objc::objc_msgSend(obj, init);
    objc::objc_msgSend(obj, doSomething);
}

