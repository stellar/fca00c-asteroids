# Asteroids <!-- omit in toc -->

Today's game will set you on a spaceship in one of the darkest corners of the
universe! It is your quest to explore this uncharted portion of the universe and
destroy 100 asteroids. Don't forget to re-fuel and upgrade your ship along the
way.

## TL;DR

Provided here are the bullet points of what you will **absolutely need** to know
to successfully play this game. This is _intentionally_ quite brief, and
everything is explained in much greater detail further on down in this document.

- Your goal is to earn `100 points` by shooting and destroying asteroids
- You will need to navigate your ship through multiple `16x16` galaxy grids
  - `6 asteroids` are contained within each galaxy
  - `2 fuel pods` are contained within each galaxy
- Use the `p_turn`, `p_move`, `p_shoot`, `p_harvest`, and `p_upgrade` functions
  to control your ship
- Turning, moving, and shooting all consume different amounts of fuel
- Upgrade your ship to improve fuel efficiency
- Upgrading consumes points

## Table of Contents <!-- omit in toc -->

<details>
<summary>Expand Table of Contents</summary>

- [TL;DR](#tldr)
- [Competition Leaderboards and Schedule](#competition-leaderboards-and-schedule)
- [Get Started with Soroban](#get-started-with-soroban)
- [How to Play](#how-to-play)
  - [Gather Your Materials](#gather-your-materials)
  - [The Game Engine Contract](#the-game-engine-contract)
  - [The Map](#the-map)
  - [Controlling Your Ship](#controlling-your-ship)
  - [Compile Your Contract](#compile-your-contract)
  - [Submit Your Ship](#submit-your-ship)
- [Useful Information](#useful-information)
  - [`GameEngine` Initialization Parameters](#gameengine-initialization-parameters)
  - [Diagonal Turns and Moves](#diagonal-turns-and-moves)
  - [Helpful Game Engine Methods](#helpful-game-engine-methods)
  - [Testing Your Contract](#testing-your-contract)
- [Suggestions and Strategies](#suggestions-and-strategies)

</details>

## Competition Leaderboards and Schedule

_Fast, Cheap, and 0ut 0f Control_ is a wild[,][,] experimental take on coding
competitions. Your goal is to submit a working contract that solves a given
problem, or performs a given task. We will track results in the following three
(3) leaderboards.

- **Fastest Submission**: On this leaderboard, the top prizes go to the entrants
  submitting a complete contract first, second, and so on. A good, old-fashioned
  race!
- **Smallest WASM**: We want to test the limits of how small these deployed
  contracts can be. We're awarding prizes for valid contracts with the very
  smallest compiled sizes.
- **Lowest Resource Use**: We are measuring resource use as the number of CPU
  instructions used during a contract's invocation. Top spots are given to
  complete contracts with the least CPU utilization.

Each time you submit a contract, it will be judged according to all leaderboards
currently accepting entries. Your position in each leaderboard is determined by
your **best** submission to _that particular_ leaderboard.

- **Yes!** You _can_ submit more than once. You probably should, if you want to
  be competitive within each leaderboard.
- **Yes!** You _can_ earn a prize from each leaderboard. Specific award amounts
  can be found in the [official fca00c rules][rules].

This round of fca00c will run according to the following schedule:

| Date       | Time (EST) | Unix Timestamp | What's Happening?                                                                    |
| ---------- | ---------- | -------------- | ------------------------------------------------------------------------------------ |
| 2023-02-15 | 7:00pm     | 1676505600     | fca00c-asteroids goes live! All leaderboards are open for entries.                   |
| 2023-02-22 | 7:00pm     | 1677110400     | Submission deadline for _Smallest WASM_ leaderboard.                                 |
| 2023-03-01 | 7:00pm     | 1677715200     | Submission deadline for _Fastest Submission_ and _Lowest Resource Use_ leaderboards. |

## Get Started with Soroban

As a _very brief_ precursor before we get too deep into the weeds, here is a
quick cheatsheet of references, links, and resources you might find useful if
you are confused by the words _Soroban_, _Stellar_, _Rust_, or anything else we
discuss.

- [Soroban Documentation][soroban-docs]: This is a great first step to learn
  more about Stellar's smart contract platform.
- [Soroban SDK Crate][soroban-sdk-crate]: The official Rust SDK for interacting
  with the Soroban platform.
- [The Rust Book][rust-book]: The definitive volume for learning the Rust
  programming language.
- [Rustlings][rustlings]: A more interactive method for learning Rust.
- [Soroban Quest][series-5]: Soroban Quest is an interactive course all about
  Soroban.
  - ⚠️ Due to Soroban's current alpha nature, this course may or may not be
    fully up-to-date, but it will provide some valuable context.
- [Stellar Quest][sq-learn]: Soroban lives on top of the Stellar network.
  Writing smart contracts for Soroban doesn't _require_ an in-depth knowledge of
  Stellar, but the context can be useful.

[soroban-docs]: https://soroban.stellar.org/docs
[soroban-sdk-crate]: https://docs.rs/soroban-sdk
[rust-book]: https://doc.rust-lang.org/book/
[rustlings]: https://github.com/rust-lang/rustlings
[series-5]: https://quest.stellar.org/soroban
[sq-learn]: https://quest.stellar.org/learn

## How to Play

### Gather Your Materials

Before you can begin writing your contract, you'll need some materials first.

1. [Setup your Soroban development environment][soroban-setup] using this guide
   from the Soroban documentation.
2. Git clone the [`stellar/fca00c-asteroids` repo][repo]. This is the
   _canonical_ source for information and materials used for this challenge.

If you are reading this document anywhere besides the [fca00c site][site] (in a
code editor, for example), you've probably already done everything above, so you
get to skip right to the front of the line. Well done!

> **Note:** The competition materials may be updated, changed, etc. from time to
> time. If you've cloned the git repository, we recommend you `git pull` often.
> If something seems to be broken, not working as expected, etc. checking for an
> up-to-date repo is an excellent first step.

### The Game Engine Contract

We have built a _Game Engine_ contract. Your task is to write a contract that
will interact with our game engine in the Soroban environment. This contract has
been included in two formats:

1. In the `contracts/_game-engine` directory, we've provided the source code for
   the contract, broken into its various modules. This version of the contract
   is **NOT** intended to be modified, or used in your contract. It's being
   provided only as a resource and reference to aid in understanding and
   problem-solving.
2. As a compiled WASM binary: `contracts/game_engine.wasm`. This is the version
   of the contract you'll want to build and test your solution with. The starter
   tests we've provided in the `solution` directory will mimic our evaluation
   environment as closely as possible.

If you want to investigate the compiled binary, the `soroban contract bindings`
command will give you a pretty decent understanding. `soroban contract bindings`
is intended to generate client bindings for a contract. This will give you great
insight into what functions exist in a given WASM file, and what arguments they
are expecting. You can run the command like this:

```bash
soroban contract bindings --wasm /path/to/game_engine.wasm --output rust
```

### The Map

The map in our asteroids game is an infinite cartesian plane (yes, this universe
is "flat," don't worry about it. You should be focusing on the game, anyway).
This plane is then divided into "galaxies." Each galaxy is a `16x16` square.
Note the difference between `16 squares` and `17 points` along either the `x` or
`y` axis that makes up each galaxy. For example, your ship starts its journey in
the galaxy that has a center coordinate of `(8, 8)`, and it is comprised of all
points within the following coordinates:

```text
BottomLeft: (0, 0)
BottomRight: (16, 0)
TopRight: (16, 16)
TopLeft: (0, 16)
```

Each galaxy will contain `6 asteroids` and `2 fuel pods`. You can only see or
shoot asteroids that are inside your current galaxy. When the time comes to
change galaxies, you must move your ship outside the boundaries of your current
galaxy.

> **Note:** It would be more technically correct to say, "Each galaxy will
> _usually_ contain `6 asteroids` and `2 fuel pods`." It is unlikely, but
> _possible_ that our game engine contract will generate a set of colliding
> coordinates for two elements (asteroids or fuel pods). In this improbable
> circumstance, a fuel pod will take priority over an asteroid, and there will
> be only one element left at this point ("erasing" the previous element). A
> galaxy like this would contain fewer than 8 map elements.

<details>
<summary>Expand Map Visualizations</summary>

Below, a single galaxy is illustrated. After the game engine has been first
initialized, your ship is placed at `(8, 8)` within the first galaxy.

![A single galaxy in the asteroids game][single-galaxy]

Even though you can only _see_ one galaxy at a time, there are neighboring
galaxies that contain their own asteroids and fuel pods. These are only
accessible to you once your ship has entered that next galaxy.

![Multiple galaxies in the asteroids game][multi-galaxy]

</details>

### Controlling Your Ship

Your vessel is outfitted with the most _bleeding-edge_ capabilities we could
cram into a starship! Your ship is capable of performing all these actions:

- **Turn**: Your ship can turn in any direction you choose. Turning does cost
  fuel, though. It will cost you the same amount of fuel, no matter which
  direction you are turning to.
  - Use the game engine's `p_turn()` method to turn your ship in the desired
    direction before you shoot or move. Each turn will cost `1 fuel`, no matter
    how far you are turning (i.e., turning 45° costs the same as turning 180°).
  - You must supply a `Direction` argument to the `p_turn()` method, specifying
    the new direction you'd like your ship to face:
    `engine.p_turn(&Direction::Left)`.
  - Remember, calling the `p_turn()` method consumes a flat cost of `1 fuel`,
    even if you turn 315°.
  - Read below for more details on [diagonal turns][diagon-alley].
- **Move**: Your ship can move any number of spaces as long as you have enough
  fuel to cover that cost. Each space you move costs fuel as well (twice as much
  fuel as each turn, mind you).
  - Use the game engine's `p_move()` method to move your ship. By default, your
    ship will move `1 space` in the direction it is facing.
  - You can (optionally) provide a number of spaces you'd like your ship to
    move: `engine.p_move(Some(4))`.
  - Remember that moving will cost `2 fuel` for every _space_ moved, no matter
    how many times `p_move()` is called, or how many spaces you provide to it as
    an argument. The calculation is **always** made on the number of spaces your
    ship moves.
- **Shoot Asteroids**: Your ship is capable of shooting asteroids to destroy
  them and earn valuable points! For each asteroid destroyed, you will be
  rewarded with `1 point`. Your ship's laser cannon has a range of `3 spaces`.
  For example, if your ship is currently located at `(53, 72)`, you could shoot
  and destroy asteroids located at `(53, 75)` or `(50, 75)`; however, an
  asteroid located at `(49, 72)` would be out of range.
  - Use the game engine's `p_shoot()` method to shoot your laser cannon in the
    direction your ship is currently facing. Each time you fire the cannon will
    cost `5 fuel`, whether you hit an asteroid or not.
  - If your ship is facing in a diagonal direction, you can shoot asteroids
    within range of that direction. This can save you fuel and execution costs
    by minimizing the number of moves needed to get within range of an asteroid.
  - You _can_ shoot asteroids sharing the same coordinates as your ship (i.e.,
    an asteroid you're "on top of").
  - You _can_ shoot multiple asteroids in one shot in the same direction,
    provided they are all within range of your ship (i.e., you shoot in a
    straight line, and any asteroids on that line will be destroyed).
- **Harvest Fuel Pods**: The amount of fuel you begin with will not be enough to
  complete this quest. Sorry! Fuel prices are quite high at the moment. So,
  along the way, you will need to harvest fuel pods to recharge your ship.
  - Use the game engine's `p_harvest()` method to harvest a fuel pod once you
    have moved your ship to the fuel pod's coordinates. Each fuel pod harvested
    will give your ship an additional `100 fuel`.
- **Upgrade**: Your ship even comes with its own upgrade feature. You can
  upgrade your ship **only once**, and you can do so at any point during your
  quest (only you can determine when the time is right). You will have to give
  up some of the points you've worked hard to earn, but after the upgrade is
  complete your ship will use half the fuel for _turning_, _moving_, and
  _shoooting_.
  - Use the game engine's `p_upgrade()` method to upgrade your ship at any point
    during your quest. Upgrading will cost `5 points`.

### Compile Your Contract

When you've written a working contract, you'll need to build a binary file that
will contain your contract's WASM byte-code. This executable file is what you
are meant to submit as "Your Ship" on the [fca00c site][site].

Compilation can be done many different ways, but we've provided a couple
commands in our `Makefile` to get you going. `make build` will produce a binary
compiled according to the "release" profile, while `make build-optimized` will
work to optimize that build and minimize the size of the `.wasm` file.

You can also choose from several different optimization strategies to produce an
efficient contract binary. There is much more nuance here than we have space to
get into fully. However, you can use the following links to get started learning
more.

- [Optimizing Builds][soroban-optimizing]: This example shows how to use the
  `soroban-cli` to optimize a compiled contract using some sensible defaults.
- [The `wasm_opt` crate][wasm-opt-crate]: This can be used to further customize
  and optimize your compiled contract.
- [The `Binaryen` toolkit][binaryen]: Binaryen is a compiler and toolchain for
  WebAssembly. This toolkit is used "under the hood" in the `wasm_opt` crate.

[soroban-optimizing]:
    https://soroban.stellar.org/docs/getting-started/hello-world#optimizing-builds
[wasm-opt-crate]: https://docs.rs/wasm-opt/latest/wasm_opt/
[binaryen]: https://github.com/WebAssembly/binaryen

### Submit Your Ship

You're finished! Really!? Sweet!! You should be proud of yourself, just for
getting to this part!

When you're ready to have your smart contract evaluated, your next step is to
[upload your compiled WASM file on the fca00c site][upload-contract]. You'll
have the option to log in and select how your name will be displayed if you land
on the leaderboard, and you can choose the file you wish to upload. We'll take
care of everything else from there!

> **Note:** The Soroban environment we run in our backend will limit your
> contract invocation to 30 seconds of runtime, and `16_000_000_000` CPU cycles.
> If your contract runs past these limits, your validation will fail.

## Useful Information

### `GameEngine` Initialization Parameters

When we are testing your solution contract, we will run it against our game
engine with a prescribed and consistent set of initialization values. These have
been documented in the `test.rs` file, but we'll include our initialization
values and some brief descriptions here, as well:

- `move_step (1)`: The number of spaces your ship will `p_move()` by default
- `laser_range (3)`: The maximum distance from which your ship's laser can
  `p_shoot()` an asteroid
- `seed (8891)`: The map's randomness is seeded with a known, consistent `u64`
  value (this ensures everyone is playing on the same map)
- `view_range (16)`: The size of each galaxy grid
- `fuel: ()`: Soroban functions can only accept a maximum of 10 parameters, so
  all the fuel parameters are collected here
  - `player_fuel (50)`: The amount of fuel your ship contains at initialization
  - `shoot_fuel (5)`: The amount of fuel consumed by the `p_shoot()` method
  - `move_fuel (2)`: The amount of fuel consumed when you `p_move()` a single
    space
  - `turn_fuel (1)`: The amount of fuel consumed by the `p_turn()` method
- `asteroid_reward (1)`: The number of points you are rewarded for destroying an
  asteroid
- `asteroid_density (6)`: The number of asteroids each galaxy will contain
- `pod_density (2)`: The number of fuel pods each galaxy will contain

### Diagonal Turns and Moves

If you take a look at the game engine's type definitions, you may notice the
`Direction` type looks like this:

```rust
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
```

From the presence of directions like `UpRight` and `DownLeft`, you could infer
(correctly) that your ship is capable of turning and moving diagonally. You can
boost your ship's efficiency by moving diagonally when that is called for.

For everyone's sake, we've simplified the calculations in the game contract. 1
unit of diagonal movement is the same as 1 unit of horizontal or vertical
movement (We know, things can get weird out there in space).

For example, if your ship is currently pointing `Up` at `(0, 0)`, and you want
to move your ship to `(3, 2)`, here's how this can help:

```text
# Fuel Use Without Diagonal Moves
p_move(2) + p_turn(Right) + p_move(3) = 4 + 1 + 6 = 11 fuel

# Fuel Use With Diagonal Moves
p_turn(UpRight) + p_move(2) + p_turn(Right) + p_move(1) = 1 + 4 + 1 + 2 = 8 fuel
```

You can also shoot asteroids in diagonal directions, as long as your ship is
facing that direction and they are within range. This is a very powerful
fuel-saving technique!

### Helpful Game Engine Methods

The game engine contract provides some helper methods so you can orient yourself
in space and monitor the other vital information about your ship:

- `p_pos()`: Returns the ship's position on the map, as a set of coordinates.
- `p_dir()`: Returns the direction your ship is currently pointed in.
- `p_fuel()`: Returns the ship's current fuel level.
- `p_points()`: Returns the player's current score.
- `get_map()`: Returns the current galaxy's map as `Map<Point, MapElement>`,
  where `MapElement` will be either an asteroid or fuel pod.

### Testing Your Contract

While writing your contract, you'll likely want to incorporate some tests along
the way. The `src/test.rs` file is there for that. We've included two test
functions to get you started.

- The `fca00c_fast()` test will test against your written contract source code,
  and is a much quicker way to iterate throughout the build process.
- The `fca00c_budget()` test will test against a _compiled_ WASM contract
  binary. Of course, for this to work, you will need a compiled contract in
  place first. You can run `make build` or `make build-optimized` (or, you can
  do it yourself with `cargo build`, provided you know how to use it).

You'll want to keep our original `fca00c_fast()` and `fca00c_budget()` functions
intact, but you can most definitely write your own. In fact, many people may
find it _easier_ to write their initial solution inside a `test.rs` file before
building the final compiled WASM binary.

Inside the testing environment, you can access very useful things from the `std`
crate, such as the `println!` macro. If you want to output any data along your
development road, it's likely that including it in a test will be the quickest
and easiest way to go about it.

> **Note:** Rust tests do not print by default. So, if you're using `println`,
> you will need to run the test like: `cargo test -- --nocapture`. This will
> send any test output to stdout.

Then, after you've written a working solution, it's pretty easy to get it moved
into your `lib.rs` file. If you want to get some kind of output from your
`lib.rs` file, you'll need to learn all about [logging][logging] and
[debugging][debugging] in Soroban. There is a lot there, but it's useful
knowledge that can help you get the information you need, right from where you
need it.

## Suggestions and Strategies

There is absolutely no shortage of interesting and unique methods you could use
to solve this problem. _Fast, Cheap, and 0ut 0f Control_ is designed to allow
for many various competitive strategies. When trying to compete for pure speed
and get the contract written before others, you'll likely care far less about
optimizing your contract's performance or execution cost. Similarly, when you're
aiming for the very cheapest execution cost, the final deployed contract size
_may_ not be a primary concern of yours. Your chosen strategy will need to
reflect the relevant competition leaderboard you're currently optimizing for.

To get you started, we are providing below some suggestions and possible
strategies you might consider using. Take them, leave them, adapt them based on
your competitive context: The choice is yours.

- This game's universe is infinite! It might be _possible_ to move in one single
  direction, blasting any asteroids that happen to be right in front of you, and
  come out victorious. That's very unlikely to work out in a cost- or
  time-efficient manner, but you're welcome to give it a shot.
- The `get_map()` function does quite a bit of calculating, and even accesses a
  few different contract storage entries. Perhaps (or, perhaps not...) it would
  be more efficient to keep track of what is in your galaxy's map on your own,
  rather than calling `get_map()` repeatedly for the same galaxy.
- Perhaps more important than anything else in this game is to think through
  carefully how you will approach navigating. How far in advance do you want to
  know the layout of your galaxy (or multiple galaxies)? Which asteroid is the
  best next target? How low is too low for your fuel before you start looking
  for a refill? All these questions can only be answered by you, and your
  answers will determine your level of success in this game.
- Before navigating to harvest a fuel pod, you might want to shoot any nearby
  asteroids first, or shoot any you pass along the way (provided you have enough
  fuel, that is).
- Once you've harvested all fuel pods and destroyed all the asteroids in a
  galaxy, it is time to move on to the next one! However, _which_ galaxy to move
  to is certainly not a trivial choice. Plan carefully how you want to navigate,
  considering which neighboring galaxy provides the closest, cheapest, and/or
  fastest path of entry.
- When you set out for a fuel pod, make sure you're heading toward the _nearest_
  one to your current location.

[,]: https://en.wikipedia.org/wiki/Fast,_Cheap_%26_Out_of_Control
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[repo]: http://www.github.com/stellar/fca00c-asteroids
[site]: https://fastcheapandoutofcontrol.com
[diagon-alley]: #diagonal-turns-and-moves
[logging]:
    https://soroban.stellar.org/docs/how-to-guides/logging#using-the-log-macro
[debugging]: https://soroban.stellar.org/docs/learn/debugging
[single-galaxy]:
    https://user-images.githubusercontent.com/2024293/217354050-405451d6-e5c5-48a4-abc4-5cc2abe5b9a3.png
[multi-galaxy]:
    https://user-images.githubusercontent.com/4383610/217380430-b00376fa-624f-4f9e-81ec-2fff88e63e37.png
[upload-contract]: https://fastcheapandoutofcontrol.com/game/asteroids/submit
[rules]: https://fastcheapandoutofcontrol.com/rules
