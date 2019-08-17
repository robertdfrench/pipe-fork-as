# pipe-fork-as
Run a function as a less-privileged user

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
