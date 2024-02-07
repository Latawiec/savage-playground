# Savage Playground Communication
This folder has all definitions of messages used in communication across Savage Playground project.

## Foreword
Project consists of 3 parts that need to exchange data:
- Game Instance
- Game Host
- Client

I managed to distill two interface layers needed to describe the communication:
- Host Management Interface
- Host Runtime Interface

### Host Management Interface
Defines interface to create, join, leave rooms. This is how client requests to start a game.

### Host Runtime Interface
Once the game is running, this interface is responsible for passing messages from client to game and backwards.

What is key to understand is this interface is absolutely _unaware what game is currently running_. 

Messages passed through Runtime Interface need to be defined as `any` type. Only the game running and the client are aware of what that type really means. Game implementation should then figure what the message is and decode it itself, same goes for Client implementation. In short - _Runtime Interface doesn't care what data is delivered, it's only job is to deliver it._

### Game Runtime Interface
This is where the actual contents of the game-client messages for Savage Playground project are defined. 