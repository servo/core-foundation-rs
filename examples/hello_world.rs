extern crate cocoa;

use cocoa::base::{NSUInteger, nil, ObjCSelector};
use cocoa::appkit::{NSApp, NSRect, NSPoint, NSSize,
					NSProcessInfo,
					NSApplication, NSApplicationActivationPolicyRegular,
					NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
					NSString,
					NSMenu, NSMenuItem};

fn main() {
	unsafe {
		let app = NSApp();
		NSApplication::setActivationPolicy_(app, NSApplicationActivationPolicyRegular);

		// create Menu Bar
		let menubar = NSMenu::new();
		let app_menu_item = NSMenuItem::new();
		NSMenu::addItem_(menubar, app_menu_item);
		NSApplication::setMainMenu_(app, menubar);

		// create Application menu
		let app_menu = NSMenu::new();
		let app_name = NSProcessInfo::processName(NSProcessInfo::processInfo());
		let quit_prefix = NSString::from_str("Quit \0");
		let quit_title = NSString::stringByAppendingString_(
			quit_prefix,
			app_name
		);
		let quit_action = "terminate:".as_selector();
		let quit_key = NSString::from_str("q\0");
		let quit_item = NSMenuItem::initWithTitle_action_keyEquivalent_(
			NSMenuItem::alloc(),
			quit_title,
			quit_action,
			quit_key
		);
		NSMenu::addItem_(app_menu, quit_item);
		NSMenuItem::setSubmenu_(app_menu_item, app_menu);

		// create Window
		let window = NSWindow::initWithContentRect_styleMask_backing_defer_(
			NSWindow::alloc(),
			NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
			NSTitledWindowMask as NSUInteger,
			NSBackingStoreBuffered,
			false
		);
		// this segfaults in invoke_msg_NSPoint_NSPoint():
		// NSWindow::cascadeTopLeftFromPoint_(window, NSPoint::new(20., 20.));
		NSWindow::center(window);
		let title = NSString::from_str("Hello World!\0");
		NSWindow::setTitle_(window, title);
		NSWindow::makeKeyAndOrderFront_(window, nil);

		NSApplication::activateIgnoringOtherApps_(app, true);
		NSApplication::run(app);
	}
}
