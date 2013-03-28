pub type id = libc::intptr_t;
pub type Class = libc::intptr_t;
pub type IMP = *u8;
pub type SEL = libc::intptr_t;
pub type Ivar = libc::intptr_t;

pub static nil : id = 0 as id;

pub extern mod objc {
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
pub fn test_nsapp() {
    let klass = str::as_c_str(~"NSApplication", |s|
        unsafe {
            objc::objc_getClass(s)
        }
    );

    let sel = str::as_c_str(~"sharedApplication", |s|
        unsafe {
            objc::sel_registerName(s)
        }
    );

    unsafe {
        let nsapp = objc::objc_msgSend(klass, sel);
        io::println(fmt!("nsapp: %d", (nsapp as int)));
    }
}

#[test]
pub fn test_custom_obj() {
    extern fn MyObject_doSomething(this : id, _sel : SEL) -> id {
        io::println(~"doSomething");
        return this;
    }

    let NSObject = str::as_c_str(~"NSObject", |s|
        unsafe {
            objc::objc_getClass(s)
        }
    );
    let MyObject = str::as_c_str(~"MyObject", |s|
        unsafe {
            objc::objc_allocateClassPair(NSObject, s, 0 as libc::size_t)
        }
    );
    let doSomething = str::as_c_str(~"doSomething", |s|
        unsafe {
            objc::sel_registerName(s)
        }
    );
    let _ = str::as_c_str(~"@@:", |types|
        unsafe {
            objc::class_addMethod(MyObject,
                                  doSomething,
                                  MyObject_doSomething,
                                  types)
        }
    );

    unsafe {
        objc::objc_registerClassPair(MyObject);
    }

    let alloc = str::as_c_str(~"alloc", |s| unsafe { objc::sel_registerName(s) });
    let init = str::as_c_str(~"init", |s| unsafe { objc::sel_registerName(s) });

    unsafe {
        let mut obj = objc::objc_msgSend(MyObject, alloc);
        obj = objc::objc_msgSend(obj, init);
        objc::objc_msgSend(obj, doSomething);
    }
}

