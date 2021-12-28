# RUST BASICS

This is a project that demonstrates most of the essential rust notation.

There are also code that illustrates some of the most important concepts regarding ownership and borrowing, which makes the rust language one of the most interesting and intriguing programming languages lately.

I really enjoyed going through all these learnings, but I must confess that rust lang makes me a little dizzy sometimes.

### BUILDING:

`cargo build`

### RUNNING

`cargo run`

## How to work with this project

The first thing to notice is that many warnings will be issued by the first time you run this project.

That is because there are many unused variables. They exist so that one could learn different rust notations.

Another important thing for choosing what to learn is:

The `main.rs` file calls many functions on the different .rs files in the `src` folder.

You should choose which concept to go through, and add/removing comments accordingly, so that the main.rs program can call only the parts that you want to learn.

The best way to use this project will be to actually call just one function at a time. 

For example:

In `main.rs`, you could comment all functions, except '`moving_ownership()`'. Therefore, you will execute the ownership concepts part of the code when running `'cargo run'`.

You should always execute a function and read the code, possibly following with a debugger.

### There are also several code lines that are commented.

This is to remind myself that some ways of writing rust is wrong. Do not remove those commented lines if you also want to be reminded!

The IDEs I have used to create this project are Intellij and CLion. Any other IDE will be of use, though.

I hope some people could enjoy as much as I did. And really have your mind going crazy around some of rust's concepts regarding memory management.

Any improvement is appreciated. Although there are more concepts and tools missing, this project here is actually done. I could create other projects to demonstrate more advanced rust concepts.

Cheers!