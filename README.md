# Feistel
Feistel cypher with predefined key and message

## Feistal function

This function take message which need to be encrypted, key for encryption and round for indication of current round of encryption.

Each turn key is xored on specific value, in this case 00000001.

Function split each side by half for next manipulations in program:
 - left value: this is right side of the messsage which would be used later as left side of cryptomessage
 - right value: this is left side of the messsage which would be used later as right side of cryptomessage

# Left value

It stay same

# Right value

This value would be chnaged specific way. First we take left value and use it in function where it would be changed. 
Here we xor cryptokey with left value. Next xor with right value.

# Final

We combine values.

## Main function

Here is written key and messsage as values.

Also, for cicle will use feistel function 6 times.
