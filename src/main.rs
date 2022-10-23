fn fiestal(message: &mut u64,key: &mut u16,round:i32){
    //Each round key would be xored
    *key ^= 0b00000001;
    //Left side of message which would be taken from right
    let left = ((*message << 8) as u16)>> 8;
    //Right side of message which would be taken from left
    let mut right = (*message >> 8) as u16;
    //Right side is placed in function for xoring with key
    let mut function = left;
    //Xor right side with key
    function ^=*key;
    //Xor left side with F
    right ^=function;
    //Switch sides
    *message = ((left << 8) |right) as u64;
    //Show rounds
    println!("round,{}", round+1);
    //Show final message
    println!("{:0b}",*message);
}
fn main() {
    let mut message:u64 = 0b0110100110101110;
    let mut key:u16=0b00001111;
    //Rounds for messsage 6
    for _i in 0..6{
        fiestal(&mut message, &mut key,_i);
    }
    println!("Answer:{:0b}",message);
}