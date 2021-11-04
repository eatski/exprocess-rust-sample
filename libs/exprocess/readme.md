# What's Exprocess?
Exprcess provides a thin framework for declaratively describing the logic of state transitions in applications, and a library that helps to apply the logic created according to the framework to your application.

## What are the benefits of using Exprocess?
- Forces a highly testable implementation.
- Implemented logic can be run anywhere.
- Provides easy state reconstruction and Redo/Undo operations.

## Let's take a look at the simple code.
Let's take a look at the core of Exprcess.  
[Source code](/libs/exprocess/core.rs)
Exprocess provides a trait called ExprocessCore. This trait requires the implementation of the following three functions.  
- init
- resolve
- reducer

For more details about each function, please refer to the comment doc.  

## Runners
Core is merely a declarative description of logic.  
Exprocess provides "Runner" as helpers to make the declarative logic work and apply it to applications.  

|||
|---|---|
| Web Client Runner | Runner that is designed to work in a browser and with multiple people sharing the state. |

# Let's take a look at this repository and the sample.
## URL
https://pick-role.web.app/
## What's this app?
In this application, you can assign "roles" to multiple participants at random.
The front-end is designed so that the roles are only visible to the assigned participants themselves.

## Let's take a look at the code.
Now that I've explained it, you should be able to see it in the code.  
The following is a list of some of the key points.
### Core  
[/src/domain/state.rs]()

### Runner Use
[/src/containers/main.rs]()


