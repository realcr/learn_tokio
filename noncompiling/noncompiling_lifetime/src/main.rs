

struct FragMsgReceiver<'a,D:'a> 
    where D: FnMut(&mut [u8]) -> &mut [u8] {
    recv_dgram: &'a mut D,
}

impl<'a,D> FragMsgReceiver<'a,D>
    where D: FnMut(&mut [u8]) -> &mut [u8] {

    fn new(recv_dgram: &'a mut D) -> Self {
        FragMsgReceiver {
            recv_dgram,
        }
    }
}

fn recv_dgram(buff: &mut [u8]) -> &mut [u8] {
    buff
}

fn constrain_handler<F>(f: F) -> F
    where F: FnMut(&mut [u8]) -> &mut [u8] {
    f
}

fn main() {
    /*
    let mut recv_dgram = constrain_handler(|buff| {
        buff
    });
    */
    // let ref_recv_dgram = &mut recv_dgram;
    // let ref_recv_dgram = &mut recv_dgram;
    let fmr = FragMsgReceiver::new(&mut recv_dgram);
    let mut my_buff = [0; 512];
    (*fmr.recv_dgram)(&mut my_buff);
}

