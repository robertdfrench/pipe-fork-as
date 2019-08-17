# Pipe Forkas
*Oh, you know those pipe forkas, always runnin' around forkin' pipes*

### Abstract
Run a function as a less-privileged user by forking and communicating the
response over a pipe. 

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
