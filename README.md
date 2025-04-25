# MrKeys

<div align="center">
  <img src="assets/mrkeys_logo.svg" width="60%" alt="MrKeys Logo">
</div>

> **Current Status**: Project exists firmly in the imagination realm... but hey, we have a name and a fancy logo, so we're practically halfway there! 😉

## About

MrKeys is a keycaster tool designed to capture and display keyboard inputs in real-time. Perfect for streamers, tutorial creators, and anyone who's ever said "let me show you how I did that" only to have people miss what keys you pressed. Built with Rust and Tauri (React/Tailwind), initially for macOS with plans to conquer other platforms later.

## Project Vision

A fun weekend project with the following goals:

* Create a universal keycaster that works across all applications (because life is too short to be restricted to just Neovim)
* Build with Rust and Tauri for cross-platform capability
* Develop initially for macOS, with Windows and Linux support planned for future releases
* Design a clean, customizable visual interface inspired by nvzone/showkeys
* Provide an intuitive, unobtrusive user experience

## Development Approach

The development of MrKeys will be:

* Livestreamed/recorded as part of a livecoding series on the [MrJakob YouTube channel](https://youtube.com/c/MrJakob) (watch me struggle in real-time!)
* A fun weekend coding project (that might stretch into several weekends, let's be honest)
* Built with Rust backend and Tauri with React and Tailwind for the frontend



## Development Roadmap

- [X] Implement global key capturing on macOS using accessibility APIs or appropriate Rust libraries
- [X] Initialize Tauri environment with React
- [X] Integrate key grabbing functionality with Tauri (addressing macOS main thread restrictions)
- [ ] Create data channel between frontend and backend for key event transmission
- [ ] Install tailwindcss for the frontend
- [ ] Design and implement clean, attractive key display styling
- [ ] Develop configuration dialog for user preferences
- [ ] Add menubar integration for quick access to settings and enable/disable functionality

## Potential Future Ideas

* Expand platform support to Windows and Linux
* Make styling customizable through CSS and React
* More complex key aggregation and visual representation of keyboard combinations

## Why Another Keycaster?

There are already several excellent open-source keycaster tools available. So why am I making another one? Well:

* **Learning experience**: A chance to tinker with Rust, Tauri, and dive into those system accessibility APIs
* **Personal use**: Something tailored for my own videos and potential future streams
* **Fun project**: Simply for the enjoyment of building something useful from scratch
* **Customization**: Designing exactly what I want

MrKeys was inspired by [nvzone/showkeys](https://github.com/nvzone/showkeys), a Neovim-specific keycaster I regularly use for my Neovim videos. After a conversation in the video comments with a viewer, I realized I really like the visual style of that tool but wanted one that isn't restricted to Neovim and can be used for everything.

While there are some great keycaster tools out there already, I thought it would be fun to create my own take on it that perfectly fits how I want to use it.



## License

The project is intended to be released under the MIT License. However, this may change depending on the licensing requirements of libraries and dependencies used during development. The final license will be determined once all dependencies are finalized.

