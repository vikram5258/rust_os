# My Rust-OS

Rust-OS is a fully Rust-based operating system built from scratch. This project aims to explore the capabilities of the Rust programming language in systems programming and to provide a learning resource for those interested in OS development.

## Features

*Note: The features listed below are planned and currently under development.*

- **Memory Management**: Efficient and safe memory management using Rust's ownership model.
- **Multitasking**: Support for multitasking with a custom scheduler.
- **File System**: Basic file system implementation.
- **Device Drivers**: Support for various hardware devices.
- **Networking**: Basic networking capabilities.

## Implementation Phases

The development of Rust-OS is divided into several phases, each focusing on different aspects of the operating system:

### Phase 1: Bootstrapping
- Set up the initial bootloader.
- Basic kernel initialization.
- Simple output to the screen.

### Phase 2: Memory Management
- Implement basic memory allocation.
- Set up paging and virtual memory.

### Phase 3: Multitasking
- Develop a custom scheduler.
- Implement context switching.

### Phase 4: File System
- Create a basic file system.
- Implement file read/write operations.

### Phase 5: Device Drivers
- Develop drivers for essential hardware (e.g., keyboard, display).
- Implement a driver model for extensibility.

### Phase 6: Networking
- Set up basic networking capabilities.
- Implement simple network protocols.

## Installation

To build and run Rust-OS, you will need the following tools:

- [Rust](https://www.rust-lang.org/tools/install)
- [QEMU](https://www.qemu.org/download/)

### Building

Clone the repository and build the project:

