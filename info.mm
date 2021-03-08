/**
 * clang++ main.mm -framework Cocoa -o test && ./test
 **/

#import <Cocoa/Cocoa.h>

@interface TestView: NSView
{
}

@end

@implementation TestView

- (id)initWithFrame:(NSRect)aFrame
{
  if (self = [super initWithFrame:aFrame]) {
  }
  return self;
}
extern "C" {
typedef enum {
    kCGContextTypeUnknown,
    kCGContextTypePDF,
    kCGContextTypePostScript,
    kCGContextTypeWindow,
    kCGContextTypeBitmap,
    kCGContextTypeGL,
    kCGContextTypeDisplayList,
    kCGContextTypeKSeparation,
    kCGContextTypeIOSurface,
    kCGContextTypeCount
} CGContextType;


CGContextType CGContextGetType(CGContextRef);
}
-  (void)drawRect:(NSRect)aRect
{
  CGContextRef cg = NSGraphicsContext.currentContext.CGContext;
  CFShow(cg);
  printf("CGContextType: %d\n", CGContextGetType(cg));
  CGColorSpaceRef color = CGBitmapContextGetColorSpace(cg);
  CFShow(color);
  exit(1);
  for (int y = 0; y<20; y++) {
  for (int x = 0; x<20; x++) {
  CGContextSetRGBFillColor(cg, 0.2, 0.6, 1.0, 0.9);
        CGContextFillEllipseInRect(cg, CGRectMake(50+x*50, 30+30*y, 200, 130));
  }
  }
}

@end

@interface TerminateOnClose : NSObject<NSWindowDelegate>
@end

@implementation TerminateOnClose
- (void)windowWillClose:(NSNotification*)notification
{
  [NSApp terminate:self];
}
@end

int
main (int argc, char **argv)
{
  NSAutoreleasePool* pool = [[NSAutoreleasePool alloc] init];

  [NSApplication sharedApplication];
  [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];

  NSRect contentRect = NSMakeRect(400, 300, 300, 200);
  NSWindow* window = [[NSWindow alloc] initWithContentRect:contentRect
                                       styleMask:NSTitledWindowMask
                                         backing:NSBackingStoreBuffered
                                           defer:NO];

  NSView* view = [[TestView alloc] initWithFrame:NSMakeRect(0, 0, contentRect.size.width, contentRect.size.height)];
    
  [window setContentView:view];
  [window setDelegate:[[TerminateOnClose alloc] autorelease]];
  [NSApp activateIgnoringOtherApps:YES];
  [window makeKeyAndOrderFront:window];

  [NSApp run];

  [pool release];
  
  return 0;
}
