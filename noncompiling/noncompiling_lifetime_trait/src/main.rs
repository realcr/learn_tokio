trait MockFutureTrait {
    type Item;
    fn get_item(self) -> Self::Item;
}

type FnTraitObject = FnMut(&mut [u8]) -> MockFutureTrait<Item=&mut [u8]>;

struct MockFuture<T> {
    item: T,
}

impl<T> MockFutureTrait for MockFuture<T> {
    type Item=T;
    fn get_item(self) -> Self::Item {
        self.item
    }
}

struct FragMsgReceiver<'a> {
    recv_dgram: &'a mut FnTraitObject,
}

struct RecvMsg<'a,'c,F>
    where F: MockFutureTrait<Item=&'c mut [u8]> {

    frag_msg_receiver: FragMsgReceiver<'a>,
    res_buff: &'c mut [u8],
    read_future: F,
}


fn main() {
    let mut recv_dgram = |buf: &mut [u8]| {
        MockFuture {
            item: buf,
        }
    };

    let fmr = FragMsgReceiver {
        recv_dgram: &mut recv_dgram,
    };
}
