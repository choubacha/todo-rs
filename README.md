# TODO api written in rust

This is a simple API written in rust using postgres as a backend. The basics
are a simple set of resources that can be created. This is a toy app to demonstrate
building a diesel and actix web app.

### Lists

A list is a collection of items that can be done.

### Items 
An item is a thing that needs to be done. When it is done, it can be updated to
that status.


## Getting Started

This app is built with local dev in mind. To stand it up, you just use docker:

```
docker-compose up
```

This will boot various services that will bring the service up. It will run a
container with the migrations to bootstrap and update the database. The database
itself is configured in as a separate container (with a configured volume for
persistance).

If you wish to run commands, use the build service as it will have cargo and other
rust tools installed. The web server, however, runs off of a compiled release target.

## Routes

```
GET     /lists            # Fetches a list of lists..
POST    /lists            # Creates a new list (takes name as an argument)
GET     /lists/:id        # Gets the list
PATCH   /lists/:id        # Updates a list (like the name)
DELETE  /lists/:id        # Deletes a list (and it's todos)

GET     /lists/:id/todos  # Gets the todos for a list
POST    /lists/:id/todos  # Creates a new todo on the list
GET     /todos/:id        # Fetchs a todo item
PATCH   /todos/:id        # Updates the todo (like the description or the state)
DELETE  /todos/:id        # Deletes the todo (like it never existed)
```
