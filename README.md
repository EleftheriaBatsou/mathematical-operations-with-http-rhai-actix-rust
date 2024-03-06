## Mathematical operations (+, *, /) with rhai, actix in Rust

Overall, this Rust project demonstrates how to integrate Actix for handling HTTP requests and Rhai for executing dynamic scripts to perform mathematical operations dynamically. It provides a simple REST API for performing addition, multiplication, and division operations via HTTP endpoints.

**Actix**

Actix is a powerful, actor-based framework for building asynchronous applications in Rust. It is well-known for its high performance and scalability, making it suitable for developing web applications, microservices, and network services. Actix provides tools and abstractions for handling HTTP requests, managing state, and orchestrating concurrent tasks.

**Rhai**

Rhai is a lightweight scripting language engine for Rust. It allows embedding scripting capabilities into Rust applications, enabling users to execute custom scripts dynamically at runtime. Rhai scripts can interact with Rust code seamlessly, providing flexibility and extensibility to applications.

#### **Explaining the Code:**
- **Imports:** The code imports necessary modules and traits from Actix and Rhai libraries to handle HTTP requests and execute Rhai scripts.

- **Route Handlers:** Three route handlers (divide, multiply, add) are defined using Actix macros (#[get("/divide/{num1}/{num2}")], #[get("/multiply/{num1}/{num2}")], #[get("/add/{num1}/{num2}")]). These handlers extract two integer parameters (num1 and num2) from the URL path using the Path extractor.

- **Rhai Integration:** Inside each route handler, a new Rhai engine is created (Engine::new()), and Rust functions (num1 and num2) are registered with the engine using register_fn. These functions return the corresponding num1 and num2 values extracted from the URL path.

- **Script Evaluation:** After registering the Rust functions, each handler evaluates a Rhai script file ("src/divide.rhai", "src/multiply.rhai", "src/add.rhai") using the eval_file method of the Rhai engine. This method executes the script and returns the result.

- **Actix Web Server:** The main function initializes an Actix HTTP server using HttpServer::new(), configures the server with route handlers, and binds it to the address "127.0.0.1:8080". Finally, the server is started using the run() method.

#### **Explanation of Rhai Scripts:**
- Each Rhai script (multiply.rhai, add.rhai, divide.rhai) defines a function (multiply, add, divide) that accepts two parameters (num1 and num2) and performs the corresponding mathematical operation.
The multiply function multiplies num1 and num2, the add function adds them, and the divide function divides num1 by num2.
- The script then calls the respective function with num1 and num2 retrieved from the Rust functions.


#### **Dependencies:**
The Cargo.toml file specifies the dependencies required for the project, including Actix (actix-web) and Rhai (rhai).

#### Notes:
1. ```cargo run``` and then open your browser
2. Type sth like: ```http://localhost:8080/multiply/5/3``` and ```http://localhost:8080/add/5/3```
3. use ctrl + c to stop the program
4. to kill a port you can use something like: sudo kill -9 `sudo lsof -t -i:8080`
