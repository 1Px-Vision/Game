# Building a Game Engine with Rust from the Ground Up

This repository provides a foundation for creating a straightforward yet effective game engine using Rust. The project is crafted to help you put into practice the concepts and skills acquired during the Rust course. By working on this project, you'll gain practical experience with Rust's advanced capabilities, such as multi-threading, macros, and integration with C code.

Local environment prerequisites
While this project has no specific dependencies on any system, it was built on a Unix-based machine. So, if you're on Windows, I'd recommend using the Windows Subsystem for Linux (WSL), so all instructions here directly apply to your system.

You'll need to have Rust installed in your machine for this project. If you haven't installed Rust yet, you can do so with:

````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

Also, because we are dealing with C code in this project, you'll need to install a C compiler on your machine. You can install the build-essential package, which includes the GNU C Compiler (GCC) and other necessary tools:

````
sudo apt update
sudo apt install build-essential
sudo apt install libssl-dev pkg-config

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
## Running the Videogame

Run the videogame with the following command:

````
make run-videogame
````
![Result_Game](https://github.com/1Px-Vision/Game/blob/main/Building%20a%20Game%20Engine%20with%20Rust%20from%20the%20Ground%20Up/Rust_Game.jpg)

Example the result on screen
````
1:Sprite { x: 756, y: 429, width: 27, height: 19, r: 174, g: 44, b: 238 }
2:Sprite { x: 161, y: 57, width: 16, height: 37, r: 20, g: 119, b: 143 }
3:Sprite { x: 463, y: 444, width: 42, height: 43, r: 84, g: 36, b: 88 }
4:Sprite { x: 410, y: 309, width: 11, height: 43, r: 100, g: 76, b: 218 }
5:Sprite { x: 118, y: 200, width: 39, height: 39, r: 110, g: 107, b: 213 }
6:Sprite { x: 548, y: 321, width: 27, height: 37, r: 62, g: 9, b: 205 }
7:Sprite { x: 81, y: 206, width: 39, height: 16, r: 144, g: 17, b: 211 }
8:Sprite { x: 168, y: 160, width: 15, height: 39, r: 54, g: 132, b: 98 }
9:Sprite { x: 199, y: 508, width: 28, height: 44, r: 205, g: 166, b: 163 }
10:Sprite { x: 355, y: 8, width: 30, height: 41, r: 40, g: 246, b: 179 }
11:Sprite { x: 671, y: 321, width: 46, height: 18, r: 35, g: 170, b: 205 }
12:Sprite { x: 202, y: 444, width: 38, height: 15, r: 202, g: 229, b: 175 }
13:Sprite { x: 10, y: 188, width: 44, height: 44, r: 60, g: 244, b: 12 }
14:Sprite { x: 243, y: 113, width: 18, height: 16, r: 164, g: 241, b: 13 }
15:Sprite { x: 670, y: 341, width: 47, height: 24, r: 109, g: 113, b: 198 }
16:Sprite { x: 301, y: 424, width: 49, height: 27, r: 58, g: 129, b: 247 }
17:Sprite { x: 573, y: 470, width: 24, height: 14, r: 62, g: 252, b: 185 }
````

## Running Ping Pong
Run Ping Pong with the following command:

make run-pingpong
