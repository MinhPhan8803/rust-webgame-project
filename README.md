# CS 128H Final Project

### Group name: Move Error

### Group members:

- Minh Phan (minhnp2), Zean(Wyatt) Huang (zeanh2), Sailaja Nallacheruvu (snall6)

### Introduction:
- Our goal is to implement a multiplayer version of Blackjack. Our primary goal would be to implement all the rules of the card game and make it so that multiple users can play one game at once. We’d also like to implement a backend to handle game states, save information about users, and save their game progress. Additionally, if we have time, one extra goal would be to implement some kind of global ranking system for all users, but that is a goal that would come after implementing the game.
- We chose this project because every member of our group was interested in topics related to networking and creating a project that connected to both that and implementing an algorithm. Our objectives with this project are to get more practice with web development and coding multiplayer games in Rust.

### System Overview:
1. Implement the Blackjack game:
  - [ ] Look into crates that are helpful to implementing the game.
  - [ ] Create game interfaces through output from terminal.
  - [ ] Implement rules of blackjack.
  - [ ] Create sample game states to test the games.
  - [ ] Implement game tutorial in the terminal.
2. Build the backend server to handle all the game states and user information:
  - [ ] Look into technology to use to build backend.
  - [ ] Implement ability to process and check moves.
  - [ ] Database for user info.
  - [ ] Database for ongoing games.
  - [ ] Allow creating rooms for different players.
3. Connect the backend server and the game through API communication:
  - [ ] REST API with CRUD(create, read, update, and delete) functionality of all the game states and user information.
  - [ ] Web sockets allow user play games against each other live.
  - [ ] Web requests error handling.
  - [ ] Connection status indicator during/at the beginning of the game.

### Possible challenges:
- Finding suitable technologies to implement the project.
- Client - server communication through web sockets/APIs.

### References:
