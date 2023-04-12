## Rust Mini Project #11: Compare two strings

- Given two strings, s1 and s2, with the same length, determine whether a permutation of s1 can break a permutation of s2 or vice versa.

- A string s1 can break s2 if every character in s1 is greater than or equal to the corresponding character in s2 in alphabetical order. In other words, s1 breaks s2 if s1 is lexicographically greater than or equal to s2. Similarly, if s2 is lexicographically greater than or equal to s1, then it is said that s2 can break s1.


### Usage

1. cargo run -- beat

2. Input two strings.

- abc and xay
<img width="437" alt="Screenshot 2023-04-12 at 3 09 31 PM" src="https://user-images.githubusercontent.com/90014065/231560480-5530699d-454a-43fe-a60e-a120ebcdc151.png">

- abx and ade
<img width="432" alt="Screenshot 2023-04-12 at 11 40 03 AM" src="https://user-images.githubusercontent.com/90014065/231559092-4ac2d28d-9829-4b29-8fc9-effc7bb60caa.png">

- xa and abced
<img width="444" alt="Screenshot 2023-04-12 at 3 09 47 PM" src="https://user-images.githubusercontent.com/90014065/231560500-a096a107-c75b-452f-adaa-d7fe30b0de8f.png">
