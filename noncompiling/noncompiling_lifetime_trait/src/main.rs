trait MockFutureTrait {
    type Item;
    // fn get_item(self) -> Self::Item;
}

struct MockFuture<T> {
    item: T,
}

impl<T> MockFutureTrait for MockFuture<T> {
    type Item=T;
    // fn get_item(self) -> Self::Item {
    //     self.item
    // }
}

/*
struct FragMsgReceiver<'a> {
    recv_dgram: &'a FnMut(&mut [u8]) -> MockFutureTrait<Item=&mut [u8]>,
}
*/

/*
struct FragMsgReceiver<'a,F>
    where F: MockFutureTrait<Item=&mut [u8]> {
    recv_dgram: &'a FnMut(&mut [u8]) -> F,
}

struct FragMsgReceiver2<'a> {
    recv_dgram: &'a FnMut(&mut [u8]) -> MockFuture<&mut [u8]>,
}
*/

struct FragMsgReceiver<'a> {
    recv_dgram: &'a for<'b> FnMut(&'b mut [u8]) -> Box<MockFutureTrait<Item=&'b mut [u8]> + 'b>,
}

/*
struct RecvMsg<'a,'c,F>
    where F: MockFutureTrait<Item=&'c mut [u8]> {

    frag_msg_receiver: FragMsgReceiver<'a>,
    res_buff: &'c mut [u8],
    read_future: F,
}
*/

/*
fn recv_dgram<'b>(buff: &'b mut [u8]) -> Box<MockFutureTrait<Item=&'b mut [u8]> + 'b> {
    Box::new(MockFuture {
        item: buff,
    })
}
*/

fn constrain_handler<F>(f: F) -> F
where F: for <'r> FnMut(&'r mut [u8]) -> Box<MockFutureTrait<Item=&'r mut [u8]> + 'r> {
    f
}


fn main() {
    let mut recv_dgram = constrain_handler(|buf| {
        Box::new(MockFuture {
            item: buf,
        }) as Box<MockFutureTrait<Item=&mut [u8]>>
    });
    // let ref_recv_dgram = &mut constrain_handler(recv_dgram);

    // let ref_recv_dgram = &mut constrain_handler(recv_dgram);
    let fmr = FragMsgReceiver {
        recv_dgram: &mut recv_dgram,
    };
}

