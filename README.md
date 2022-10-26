# SHA-1 Hash Cracker

SHA-1 is a hash function used  by a lot of old websites used to store the passwords of the users. In theory a hash password can't be recovered from it's hash and thus by storing the hash in their databases, a website can asset that a given user have the knowledge of it's password without storing the password in cleartext. So if the website's database is breached, there is no way to recover the passwords and access the user's data.


## Objective
Let's imagine a scenario where we just breacked a website and we want to recover the credentials of the users in order to gain access to their accounts. This is where a "hash cracker" is useful. A hash cracker is a proegram that will try a lot of different hash in order to find the original password.


## Compilation
```
git clone https://github.com/manuelinfosec/rust-sha1-cracker
cd rust-sha1-cracker
cargo buiild --release
```

If you don't have rust installed, you can use the pre-built optimized binary located in the `bin` folder.


## Usage

```
sha1_cracker wordlists/wordlist.txt <hash>
```
**Example hash**: 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53


## Features:
- Computes the most similar password to the target hash using the normalized levenshtein algorithm.


## Upcoming
- More verbose output during runtime.