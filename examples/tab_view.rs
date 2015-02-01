extern crate cocoa;

use cocoa::base::{NSUInteger, selector, nil, YES, NO};
use cocoa::appkit::{NSApp, NSRect, NSPoint, NSSize,
					NSAutoreleasePool, NSProcessInfo,
					NSApplication, NSApplicationActivationPolicyRegular,
					NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
					NSClosableWindowMask,NSResizableWindowMask,NSMiniaturizableWindowMask,NSUnifiedTitleAndToolbarWindowMask,
					NSString,
					NSMenu, NSMenuItem,
					NSTabView, NSTabViewItem};


fn main() {
	unsafe {
		
		// create a tab View
		let tab_view = NSTabView::new(nil).initWithFrame_(NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)));

		// create a tab view item
		let tab_view_item = NSTabViewItem::new(nil).initWithIdentifier_(NSString::alloc(nil).init_str("TabView1\0"));
		
		tab_view_item.setLabel_(NSString::alloc(nil).init_str("Tab view item 1  \0"));
		tab_view.addTabViewItem_(tab_view_item);

		//create a second tab view item
		let tab_view_item2 = NSTabViewItem::new(nil).initWithIdentifier_(NSString::alloc(nil).init_str("TabView2\0"));
		
		tab_view_item2.setLabel_(NSString::alloc(nil).init_str("Tab view item 2  \0"));
		tab_view.addTabViewItem_(tab_view_item2);

		//Create the app and set the content.
		let app = create_app(NSString::alloc(nil).init_str("Tab View\0"),tab_view);
		app.run();
	}
}

unsafe fn create_app(title:i64,content:i64) -> i64{
	let _pool = NSAutoreleasePool::new(nil);

	let app = NSApp();
	app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

	// create Menu Bar 
	let menubar = NSMenu::new(nil).autorelease();
	let app_menu_item = NSMenuItem::new(nil).autorelease();
	menubar.addItem_(app_menu_item);
	app.setMainMenu_(menubar);

	// create Application menu
	let app_menu = NSMenu::new(nil).autorelease();
	let quit_prefix = NSString::alloc(nil).init_str("Quit \0");
	let quit_title = quit_prefix.stringByAppendingString_(
		NSProcessInfo::processInfo(nil).processName()
	);
	let quit_action = selector("terminate:");
	let quit_key = NSString::alloc(nil).init_str("q\0");
	let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
		quit_title,
		quit_action,
		quit_key
	).autorelease();
	app_menu.addItem_(quit_item);
	app_menu_item.setSubmenu_(app_menu);

	// create Window
	let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
		NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
		NSTitledWindowMask as NSUInteger | NSClosableWindowMask as NSUInteger | NSResizableWindowMask as NSUInteger | NSMiniaturizableWindowMask as NSUInteger | NSUnifiedTitleAndToolbarWindowMask as NSUInteger,
		NSBackingStoreBuffered,
		NO
	).autorelease();
	window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
	window.center();
	
	window.setTitle_(title);
	window.makeKeyAndOrderFront_(nil);

	window.setContentView_(content);

	app.activateIgnoringOtherApps_(YES);
	return app;
}


