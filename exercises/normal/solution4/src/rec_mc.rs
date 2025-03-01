pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut amount=amount;
    let money=vec![100,50,30,20,10,5,2,1];
    let mut num=0;
    for i in money {
        if amount/i != 0 {
            num += amount/i;
            amount%=i
        }
    }
    num
}
