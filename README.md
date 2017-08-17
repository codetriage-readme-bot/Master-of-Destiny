<pre>
_________ __
/   _____/|  | __ ___.__.  ____________  _____     ____    ____
\_____  \ |  |/ /<   |  | /  ___/\____ \ \__  \  _/ ___\ _/ __ \
/        \|    <  \___  | \___ \ |  |_> > / __ \_\  \___ \  ___/
/_______  /|__|_ \ / ____|/____  >|   __/ (____  / \___  > \___  >
\/      \/ \/          \/ |__|         \/      \/      \/
</pre>


# Skyspace

[![Code Triagers Badge](https://www.codetriage.com/christopherdumas/skyspace/badges/users.svg)](https://www.codetriage.com/christopherdumas/skyspace)

## A Survival City Building Roguelike

![Main Screenshot](assets/screenshots/mainshot.png?raw=true "A particularly interesting generated world.")
![](assets/screenshots/new_terrain_generator_algorithm.png?raw=true)

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

## Feature List (1000ft view)

After about very two-three stages, I'll do some kind of codebase
cleanup, from a refactor to prehaps a performance issue.

- [x] Overall world generation (mountains, hills, plants, cliffs, water)
- [x] UI
- [x] Physics
- [x] Time
- [x] Seasons
- [ ] Environmental effects (biomes, weather)
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
- For information on contributing, see [CONTRUBITING.md](https://github.com/christopherdumas/Skyspace/tree/master/CONTRIBUTING.md)
- For contact information, see [CREDITS.md](https://github.com/christopherdumas/Skyspace/tree/master/CREDITS.md)
