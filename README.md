## README

### Many-Time Pad Attack Solution

This project provides a solution to the Many-Time Pad attack challenge described in the [Polkadot Blockchain Academy's syllabus](https://github.com/Polkadot-Blockchain-Academy/pba-content/blob/main/syllabus/1-Cryptography/materials/many-time-pad.md).

### Code Explanation

The code consists of three main functions:

1. **xorstr**: XORs a given plaintext string with a segment of the ciphertext.
2. **cribdrag**: Performs a crib-dragging attack by sliding the crib across the ciphertext and applying the XOR operation.
3. **main**: The entry point of the program that defines the ciphertext and the crib, and then calls the `cribdrag` function.
