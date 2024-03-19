# Generating public key from private key with BLS12-381 curve 

I am using the BLS12-381 implemented in `lambdaworks` library.
After getting the generator, we can find the public key by multiplying the generator with the private key.
`K = k * G`, where `K` is the public key, `k` is the private key and `G` is the generator.


This is for Lambda Sparkling Water Bootcamp in Cryptography.

The public keys coordinates for '0x6C616D6264617370' are :
`"9777B770DF7E2CA2B09FFFD0B9E450824683DB166102731B2D6AC2D8C626464D210ABC9D563162CDDFA71CAF72DF51" and "B4282D054C6007D8732CBCA330AAF798F4253D2E1B6A42BE1E4A306C82C18C83DCAC6ADA75FB47838BF256C88DC4AC6"`

