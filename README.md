# Dining Philosophers

## **⚠️ Disclaimer: This project is for educational purposes only.**

This is a simulation of the classic Dining Philosophers problem written in Rust programming language.

## Problem Statement

The Dining Philosophers problem is a classic computer science problem that illustrates the challenges of synchronizing access to shared resources in concurrent systems. In this problem, five philosophers are seated around a table with five forks. The philosophers alternate between thinking and eating, and they can only eat when they have both their left and right forks. The challenge is to ensure that no philosopher starves and that there are no deadlocks or race conditions.

## Solution

The solution to the Dining Philosophers problem implemented in this code uses `Mutex` and `Semaphore` objects from the standard library to synchronize access to shared resources and prevent race conditions.

## Usage

`cargo run`

## Dependencies

The program requires the following dependencies:

- [semaphore](https://crates.io/crates/semaphore)
- [core](https://doc.rust-lang.org/core/)

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
