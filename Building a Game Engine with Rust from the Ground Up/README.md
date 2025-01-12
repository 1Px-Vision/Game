# Building a Game Engine with Rust from the Ground Up

This repository provides a foundation for creating a straightforward yet effective game engine using Rust. The project is crafted to help you put into practice the concepts and skills acquired during the Rust course. By working on this project, you'll gain practical experience with Rust's advanced capabilities, such as multi-threading, macros, and integration with C code.

Local environment prerequisites
While this project has no specific dependencies on any system, it was built on a Unix-based machine. So, if you're on Windows, I'd recommend using the Windows Subsystem for Linux (WSL), so all instructions here directly apply to your system.

For this project, you'll need to have Rust installed in your machine. If you haven't installed Rust yet, you can do so with:

````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

Also, because we are dealing with C code in this project, you'll need to have a C compiler installed on your machine. You can install the build-essential package, which includes the GNU C Compiler (GCC) and other necessary tools:

````
sudo apt update
sudo apt install build-essential

````
Finally, you'll need to have GLFW installed in your machine. GLFW is a C library that will be the foundation of our game engine. You can install it with:

````
sudo apt install libglfw3 libglfw3-dev

````

## Running the Test C Game

![media](https://github.com/1Px-Vision/Game/blob/main/Building%20a%20Game%20Engine%20with%20Rust%20from%20the%20Ground%20Up/img.png)

To start with your project, clone this repository to your local machine:

````
git clone https://github.com/udacity/intro-to-rust-starter.git

````

To ensure you are set up correctly, you can run the test C game that comes with this project. You can build and run the test game with:

````
cd intro-to-rust-starter/starter
make run-c
````
