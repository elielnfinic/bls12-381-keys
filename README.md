# Generating public key from private key with BLS12-381 curve 

I am using the BLS12-381 implemented in `lambdaworks` library.
After getting the generator, we can find the public key by multiplying the generator with the private key.
`K = k * G`, where `K` is the public key, `k` is the private key and `G` is the generator.


This is for Lambda Sparkling Water Bootcamp in Cryptography