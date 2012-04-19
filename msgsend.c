#include <stdint.h>
#include <objc/message.h>

void
msgSend1IdHACK(id theReceiver, SEL theSender, id id) {
  objc_msgSend(theReceiver, theSender, id);
}
