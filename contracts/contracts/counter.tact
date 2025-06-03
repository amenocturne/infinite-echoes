message Add {
    amount: Int as uint32;
}

contract Counter(
    id: Int as uint32,
    counter: Int as uint32,
) {
    receive() {
        cashback(sender());
    }

    receive(msg: Add) {
        self.counter += msg.amount;
        cashback(sender());
    }

    get fun counter(): Int {
        return self.counter;
    }

    get fun id(): Int {
        return self.id;
    }
}
