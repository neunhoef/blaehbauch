# Blaehbauch

The program in this repository allocates a lot of RAM to force out
mmapped files and buffers in the Linux buffer cache.

Usage:

```blaehbauch GIGABYTES DELAY1 DELAY2```

The program will allocate and use the number of gigabytes given and
will delay for `DELAY1` (in seconds) between any two gigabytes it
allocates. At the end it will delay for `DELAY2` (in seconds).

Without arguments it will allocate until it is killed.
