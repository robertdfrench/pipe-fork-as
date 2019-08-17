# Pipefork As
*Run a function as a less-privileged user by forking and communicating the
response over a pipe.*

### Rationale
* `fork` is faster than people think (`exec` is the expensive part of that duo)
* Modern Unixes support Copy-on-write, so forking doesn't strictly use more RAM
* Maybe the operating system is better at restricting user privileges than your web framework?

### Running the Demo
You need to run the demo as root, because root can become another user. So
absolutely do not run this demo until you have read and audited the code for
yourself.

```bash
git clone git@github.com:robertdfrench/pipe-fork-as.git
cd pipe-fork-as
sudo cargo run $USER
```

### What's going on?
```console
                                                                  
              ┌─────────────────┐                                 
              │ Receive request │                                 
              │     (root)      │                                 
              └─────────────────┘                                 
                       │                                          
                       │                                          
                       ▼                                          
              ┌─────────────────┐                                 
              │   Create pipe   │                                 
              │     (root)      │                                 
              └─────────────────┘                                 
                       │                                          
                       │                                          
                       ▼                                          
                       Λ                                          
                      ╱ ╲                                         
                     ╱   ╲                                        
                    ╱     ╲                                       
                   ╱       ╲                                      
                  ╱         ╲             ┌──────────────────────┐
                 ▕   Fork    ▏───────────▶│  Child Proc (root)   │
                  ╲         ╱             └──────────────────────┘
                   ╲       ╱                          │           
                    ╲     ╱                           │           
                     ╲   ╱                            ▼           
                      ╲ ╱                 ┌──────────────────────┐
                       V                  │    setuid(alice)     │
                       │                  └──────────────────────┘
                       │                              │           
                       │                              │           
                       │                              ▼           
                       │                  ┌──────────────────────┐
                       │                  │Business Logic (alice)│
                       │                  └──────────────────────┘
                       │                              │           
                       │                              │           
                       ▼                              ▼           
              ┌─────────────────┐         ┌──────────────────────┐
              │read pipe (root) │◀────────│write to pipe (alice) │
              └─────────────────┘         └──────────────────────┘
                       │                              │           
                       │                              │           
                       ▼                              ▼           
              ┌─────────────────┐         ┌──────────────────────┐
              │ respond (root)  │         │         exit         │
              └─────────────────┘         └──────────────────────┘
```
