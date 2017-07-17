## Contributing (contact info below)

### Code Style

1. When contributing to the codebase, try to adhere to the [Rust Style Guide](https://aturon.github.io/).
1. Use the most basic feature or abstraction that will do the job.
1. Act like a compression tool: eliminate as much repition, but do not initially design for reuse (that leads to over-engineered code)
1. Do not use Object Oriented Programming, though objects (through structs and traits) are not outlawed.
1. Avoid overly functional programming (Monads, lazy lists/vectors) but use closures and iterators as much as possible
1. Avoid interative solutions, prefer recursive
1. Avoid imperitive code, prefer functional
1. Use expressions as much as possible
1. Mutate objects rarely, and variables as little as possible
1. Avoid types that go on the heap: Prefer references to Boxes, etc.
1. Avoid lots of mutability
1. Any property on a struct that could be used outside it should be public. Getter and setter methods are not needed.
1. Getter and setter methods are not needed because code should try as much as possible not to mutate an object without using its own custom methods.
1. Do not abstract out or make a reusable system for something until it occurs at least twice.
1. In keeping with rule 2, do not use macros unless they are the only option.
1. Function names should be `verb_noun`, `verb` or `verb_noun_noun`.
1. Variables should for the most part be one word. Two at most.

If code does not adhere to these principles (within reason, we're not evil!) it will not be accepted. That said, we'll try to help you out as much as possible, and Christopher will probably just fix it.

### Writing Style

1. Use "we."
1. Keep writing style formal.
1. Avoid "you."


### Who to talk to

Speak with Christopher if you:

- Want to contribute to the codebase
- Want to discuss simulation design
- Want to contribute to the website

Speak with William if you:

- Want to contribute graphics
- Want to discuss interface design or overall game design
- Want to contribute sound/music.

### Required knowledge
Be able to do/use...

All of:
- Git
- GitHub
- Markdown

One of:
- Music
- Sound effects
- Graphics
- Game design
- UI design
- Marketing
- Web design
- Rust programming
- Lua programming
