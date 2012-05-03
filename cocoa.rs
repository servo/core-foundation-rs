
fn msgSend1Id(theReceiver: base::id, theSelector: base::SEL, id: base::id) {
    msgsend::msgSend1IdHACK(theReceiver, theSelector, id);
}

#[link_args = "-L. -lmsgsend"]
#[nolink]
native mod msgsend {
    fn msgSend1IdHACK(theReceiver: base::id, theSelector: base::SEL, id: base::id);
}
