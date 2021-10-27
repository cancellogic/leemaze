# leemaze
Leemaze is code that solves binary (path or blocked) 2,3,4 and 5D mazes with Lee's maze algorithm.  Its written in rust and uses Lee's
algorithm to find shortest paths.  (maybe best described as shortest path solved by outward growing shortest liquid/flood fill)

What is a 5D maze?  Its a lot like like your morning commute.  The road might seem 3d, but your choices change with time as the trafic
 moves and allowed paths are blocked by cars,  children, pedestrians, trucks, busses, tractors, trains, drawbridges, construction, livestock, pets.
 And that might seem like a 4D maze, but if you are a good driver, you are considering where blocking things might be in the future
 (like erratic drivers) and making adjustments to avoid probabilistic futures that have more poor outcomes.  
 
So your morning commute is a 5D rat race.
 
#Why?  
I find certain bits of math and programming beautiful.  The juggling of pointers in Lee's algorithm written in Rust seemed like a
fun thing to try in when I was a rust beginner- and if I couldn't do Lee's algorithm in Rust-Lang - my opinion was Rust wasn't for me.  
But Rust-lang won against my doubts!  And so I share.

#How to use:
I'm writing this readme a few years after coding Leemaze.  As I recall, (1) make a binary bit maze.  (2) make a list of allowed moves
and (3)  define a start and goal.  (4) run leemaze funciton and get back a path of allowed moves that take you toward your goal.

The code has better documentation,
Enjoy - Dusty D.
 
 
