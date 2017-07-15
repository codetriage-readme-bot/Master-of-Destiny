<pre>
_________ __
/   _____/|  | __ ___.__.  ____________  _____     ____    ____
\_____  \ |  |/ /<   |  | /  ___/\____ \ \__  \  _/ ___\ _/ __ \
/        \|    <  \___  | \___ \ |  |_> > / __ \_\  \___ \  ___/
/_______  /|__|_ \ / ____|/____  >|   __/ (____  / \___  > \___  >
\/      \/ \/          \/ |__|         \/      \/      \/
</pre>


# Skyspace
## A Survival City Building Roguelike

In *Skyspace* you are the first settler on a hostile planet in a
far-removed star system. You are all alone, but if you succed in
showing your homeworld that this new frontier is habitable and
profitable, more settlers will join you.

*Skyspace* is a strategically deep, complex, and permissive
world. While it may be complex, with intricate systems allowing you to
do largely whatever you want that makes sense, it aims to have a
simple, useable user interface. It wants to be your sandbox: there are
as few restrictions as possible on what you can do. You start with
only a few tools in your toolbox and through your own prowess, smarts
and enginuity you can expand your abilities. There's no limit to how
far you can go!

In *Skyspace* you start out with a single character. This
represents your first settler. You designate jobs and your settler
will proceed to do them. Your goal for the first part of the game is to
survive. If you keep your settler alive long enough, others will start
coming to help you. Each character has their own special talents, but
they can learn new ones as you assign jobs. You can tell each
character what jobs they're allowed to do, and which they are not. By
default, they'll be allowed to do some basic things to keep themselves
alive and help others, as well as their beginning talents. As they
gain skill in certain areas, they'll start being able to expand to
similar areas, as well as completing the jobs they're talented at
faster.

In *Skyspace* your entire environment is interacive. You can
cut down trees, gather plants, delve into mountains, re-channel
rivers, dig holes, harness the flow of magma from underground and
build any number of things. Plants have different medicinal properties
and stones have different values and strengths. Woods have different
values, strengths, and weights, and water can be varying levels of
dirty. Rope, levers, pressure plates, mills, wheels, drawbridges,
waterwheels, corkscrews, tracks, steam and plenty of other mechanical
elements will be added eventually to allow for even more fun.

When managing a *Skyspace* settlement, as the seasons come and go you
must provide food, shelter, occupation, and creature comforts for your
characters, or they might die or go insane. You must also defend not
only against the elements but also against beasts and malevolent
aliens.

## Installation

For now, since the project is currently in development, we do not have a binary executable. Instead you will have to...

- Install SDL2
- Install Rust
- Install Cargo (should come with Rust)
- Download the project
- Run `cargo run` in the project's directory

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

## Feature List (1000ft view)

After about very two-three stages, I'll do some kind of codebase
cleanup, from a refactor to prehaps a performance issue.

- [x] Overall world generation (mountains, hills, plants, cliffs, water)
- [x] UI
- [x] Physics
- [ ] Time
- [ ] Seasons
- [ ] Animal AI (eating, sleeping, breading, hunting)
- [ ] Settler AI (same as animal, but with social abilities)
- [ ] Destrutive directives (chop down trees, delve, etc) for settler AI
- [ ] Settler health and sanity
- [ ] Movement, place designation for settler AI
- [ ] Constructive directives for settler AI
- [ ] Workshops, constructions
- [ ] Machenery

## More Info

- [Screenshots](https://github.com/christopherdumas/Skyspace/tree/master/assets/screenshots) are available.
- Christopher can be emailed at any time.

## Contacts

Authors:

- William Trovinger
> Github: @mrgrish<br>
> Email: william.trovinger@icloud.com<br>
> Job: Graphic artist and UX/UI designer, game interface/mechanics designer<br>

- Christopher Dumas
> Github: @christopherdumas<br>
> Email: christopherdumas@gmail.com<br>
> Job: Lead programmer and game simulation/mechanics expert<br>

- Thomas Trovinger
> Github: @dufaloid<br>
> Job: Lead music and sound director<br>
