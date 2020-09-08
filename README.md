# Handle

## What is it

This is a fun little hobby project I worked on for a few days. It's inspired by What3Words, and aims to reversibly convert phone numbers to a handle (like a call-sign).

A handle is strutured as a series of adjectives followed by a noun, so a conversion looks something like this: 

`+441234567890 => secured bleakest historical brutish confined tinfoil`

and changing any character results in a completely different handle, for example: 

`+441234567891 => yellower memorable skilled unsettling approachable snowman`

My goals for this project were to for it to create funny handles, work with the the vast majority of phone numbers, including special characters like brackets and + symbols, and for the output to change significantly with any small change to the input.

## How does it work

The way it works is by first encoding digits and valid special characters into hexadecimal, and creating a purely numerical representation of the phone number string. One quirk of this is that to preserve leading zeros, zeros also have to be converted to a non-zero encoding. 

Then, using a Feistal block cypher, the number is reversibly jumbled with some random keys in such a way that small changes to the input creates large changes in the output. Because I used a 64 bit int (32 bit was too small in most cases) to store the output, it converts to a number in this range, which means the phrases are quite long on average. This could be improved upon by dynamically using smaller int sizes. 

Then the output number is converted to a phrase with some modulo arithmetic and out pops a handle!

## What did I learn

It was a good exercise in building a CLI in Rust, including some simple file system operations and doing some unusual operations. This project required a combination of encryption and encoding methods that I've not come across before, so it was great to learn more about them. In particular I found Feistal block cyphers really interesting and elegant. 