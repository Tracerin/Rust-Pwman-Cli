# Cli Password Manager
This pwman uses saved images to generate passwords based on a seeded random number generator. The saved information is all accessible in a stored json file, however, you will not be able to find the password unless you use give the app your pin -- different pins have different passwords. The program has 3 example apps stored, also having the ability to delete or add new apps / passwords as one pleases.
### Important Note:
Currently only works on linux, uses the string command to get a string of valid characters to iterate through from the images. I am planning to rewrite that functionality natively in Rust, but am happy with the current program for my uses. Writing this was a fun excuse to learn Rust, so please forgive the mess.
