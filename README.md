# Divisibility
This repository contains simple recursive algorithms that test the divisibility of numbers. I recently laid my hands on this machine, and I thought it to be a good opportunity to practice Rust on. 

## Divisibility Test for 3
A number is divisible by three if the sum of its digits is divisible by three. The recursive algorithm will be as follows: 
1. A function will take a string of numbers
2. It adds the digits in the numbers represented by the ASCII characters in the string. 
3. If their sum is greater than 9, the sum is converted back into a string, and the function is called again, with the new sum as the string argument. 
4. If the sum is not equal to 3, 6, or 9, then the original number is not divisible by three, and the program shall return false. 
5. Otherwise, the number is divisible by three, and the program shall return true.