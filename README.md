<p align="center">
  <img src="https://i.ibb.co/dGXXcPN/Magi-OSLOGO-1.png" width="500" title="hover text">
  
  <h1>What is magi os?</h1>
  <p>This personal project is about creating a small operating system in the Rust programming language. 
  <a href="https://os.phil-opp.com/">Original blog<a/> </p>
  <p>To write an operating system kernel, we need code that does not depend on any operating system features. This means that we can't use threads, files, 
  heap memory, the network, random numbers, standard output, or any other features requiring OS abstractions or specific hardware. Which makes sense, since w
  e're trying to write our own OS and our own drivers.

  This means that we can't use most of the Rust standard library, but there are a lot of Rust features that we can use. For example, we can use iterators,
  closures, pattern matching, option and result, string formatting, and of course the ownership system. These features make it possible to write a kernel
  in a very expressive, high level way without worrying about undefined behavior or memory safety.

  In order to create an OS kernel in Rust, we need to create an executable that can be run without an underlying operating system. 
  Such an executable is often called a “freestanding” or “bare-metal” executable .

  </p>
</p>
