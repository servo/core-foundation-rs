#include <stdint.h>
#include <objc/message.h>

void
msgSend1IdHACK(id theReceiver, SEL theSender, id id) {
  printf("msgSend1IdHACK\n");
  printf("%ld\n", theReceiver);
  printf("%ld\n", theSender);
  printf("%ld\n", id);
  objc_msgSend(theReceiver, theSender, id);
}
