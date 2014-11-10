// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <AppKit/AppKit.h>
#include <Foundation/Foundation.h>
#include <objc/message.h>
#include <stdint.h>
#include <stdio.h>

double invoke_msg_double(id theReceiver, SEL theSelector) {
    double (*f)(id self, SEL op, ...) = (void *)objc_msgSend_fpret;
    return f(theReceiver, theSelector);
}

id invoke_msg_id(id theReceiver, SEL theSelector) {
    return objc_msgSend(theReceiver, theSelector);
}

id invoke_msg_id_id(id theReceiver, SEL theSelector, id a) {
    return objc_msgSend(theReceiver, theSelector, a);
}

id invoke_msg_id_NSRect(id theReceiver, SEL theSelector, NSRect *rect) {
    return objc_msgSend(theReceiver, theSelector, *rect);
}

id invoke_msg_id_id_SEL_id(id theReceiver, SEL theSelector, id a, SEL b, id c) {
    return objc_msgSend(theReceiver, theSelector, a, b, c);
}

id invoke_msg_id_id_id_id_id_id(id theReceiver, SEL theSelector, id a, id b, id c, id d, id e) {
    return objc_msgSend(theReceiver, theSelector, a, b, c, d, e);
}

id invoke_msg_id_NSRect_ulong_ulong_bool(id theReceiver, SEL theSelector, NSRect a, unsigned long b, unsigned long c, bool d) {
	return objc_msgSend(theReceiver, theSelector, a, b, c, d);
}

NSInteger invoke_msg_NSInteger(id theReceiver, SEL theSelector) {
    NSInteger (*f)(id self, SEL op) = (void *)objc_msgSend;
    return f(theReceiver, theSelector);
}

long invoke_msg_long(id theReceiver, SEL theSelector) {
    return (long)objc_msgSend(theReceiver, theSelector);
}

void invoke_msg_void(id theReceiver, SEL theSelector) {
    objc_msgSend(theReceiver, theSelector);
}

void invoke_msg_void_bool(id theReceiver, SEL theSelector, bool a) {
    objc_msgSend(theReceiver, theSelector, a);
}

void invoke_msg_void_id(id theReceiver, SEL theSelector, id id) {
    objc_msgSend(theReceiver, theSelector, id);
}

void invoke_msg_void_NSInteger(id theReceiver, SEL theSelector, NSInteger a) {
    objc_msgSend(theReceiver, theSelector, a);
}

void invoke_msg_void_NSPoint(id theReceiver, SEL theSelector, NSPoint a) {
    objc_msgSend(theReceiver, theSelector, a);
}

void invoke_msg_void_NSSize(id theReceiver, SEL theSelector, NSSize a) {
    objc_msgSend(theReceiver, theSelector, a);
}

void invoke_msg_void_NSRect_bool(id theReceiver, SEL theSelector, NSRect a, bool b) {
    objc_msgSend(theReceiver, theSelector, a, b);
}

void invoke_msg_void_NSWindowOrderingMode_NSInteger(id theReceiver, SEL theSelector, NSWindowOrderingMode a, NSInteger b) {
    objc_msgSend(theReceiver, theSelector, a, b);
}

bool invoke_msg_bool(id theReceiver, SEL theSelector) {
    return objc_msgSend(theReceiver, theSelector);
}

bool invoke_msg_bool_long(id theReceiver, SEL theSelector, long a) {
    return objc_msgSend(theReceiver, theSelector, a);
}

CGFloat invoke_msg_CGFloat(id theReceiver, SEL theSelector) {
    CGFloat (*f)(id self, SEL op) = (void *)objc_msgSend;
    return f(theReceiver, theSelector);
}

NSPoint invoke_msg_NSPoint_NSPoint(id theReceiver, SEL theSelector, NSPoint point) {
    NSPoint (*f)(id self, SEL op, NSPoint p) = (void *)objc_msgSend;
    return f(theReceiver, theSelector, point);
}

NSRect invoke_msg_NSRect(id theReceiver, SEL theSelector) {
    NSRect (*f)(id self, SEL op) = (void *)objc_msgSend;
    return f(theReceiver, theSelector);
}

NSRect invoke_msg_NSRect_NSAlignmentOptions(id theReceiver, SEL theSelector, NSRect rect, NSAlignmentOptions options) {
    NSRect (*f)(id self, SEL op, NSRect r, NSAlignmentOptions opts) = (void *)objc_msgSend;
    return f(theReceiver, theSelector, rect, options);
}

NSRect invoke_msg_NSRect_NSRect(id theReceiver, SEL theSelector, NSRect rect) {
    NSRect (*f)(id self, SEL op, NSRect r) = (void *)objc_msgSend;
    return f(theReceiver, theSelector, rect);
}

NSSize invoke_msg_NSSize(id theReceiver, SEL theSelector) {
    NSSize (*f)(id self, SEL op) = (void *)objc_msgSend;
    return f(theReceiver, theSelector);
}
