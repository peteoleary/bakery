# Lamport's Bakery

This repo implements Lamport's Bakery algorithm for thread synchronization using only shared memory: https://lamport.azurewebsites.net/pubs/bakery.pdf

This requires the use of unsafe code in Rust to defeat the compiler's extensive checks for thread safety. I tried lots of different ways to work around the compiler checks but failed. In retrospect, I should have pushed to git some of my failures because they were very informative. Read more about Rust thread safety here https://doc.rust-lang.org/book/ch16-03-shared-state.html

Output should look something like below. The important thing to note is that there is never any overlap between a processors' `begin critical section` and `end critical section` lines. This is thread safety done without low level OS or processor primitives!

```
processor #0 number is 1
processor #3 number is 4
processor #2 number is 3
processor #4 number is 5
processor #1 number is 2
> processor #0 begin critical section
> processor #0 end critical section
> processor #1 begin critical section
processor #0 number is 6
> processor #1 end critical section
> processor #2 begin critical section
processor #1 number is 7
> processor #2 end critical section
> processor #3 begin critical section
processor #2 number is 8
> processor #3 end critical section
> processor #4 begin critical section
processor #3 number is 9
> processor #4 end critical section
> processor #0 begin critical section
processor #4 number is 10
> processor #0 end critical section
```