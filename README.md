# Cli Password Manager
This pwman uses saved images to generate passwords based on a seeded random number generator. The saved information is all accessible in a stored json file, however, you will not be able to find the password unless you use give the app your pin -- different pins have different passwords. The program has 3 example apps stored, also having the ability to delete or add new apps / passwords as one pleases.
### Important Note (updated):
Should work on all platforms, wrote a simple function similar to the strings command.

~~Currently only works on Linux/Unix-based operating systems in conjunction with the bash terminal, using the string command to get a string of valid characters to iterate through from the images. I am planning to rewrite that functionality natively in Rust, but am happy with the current program for my uses. Writing this was a fun excuse to learn Rust, so please forgive the mess.~~
