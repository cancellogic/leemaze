# leemaze
Leemaze is code that solves binary (path or blocked) 2,3,4 and 5D mazes with Lee's maze algorithm. (https://en.wikipedia.org/wiki/Lee_algorithm)   This code written in rust and when given a boolean vec! maze, set of allowed moves, start and end points, finds the first shortest path as a sequence of allowed moves. 

What is a 5D maze?  Its a lot like like your morning commute.  The road might seem 3d, but your choices change with time as the trafic
moves and allowed paths are blocked by cars,  children, pedestrians, trucks, busses, tractors, trains, drawbridges, construction, livestock, pets, millions of frogs (I'm looking at you Waterville Wisconsin).  And that might seem like a 4D maze, but if you are a good driver, you are considering where blocking things might be in the future (like erratic drivers) and making adjustments to avoid probabilistic futures that have more poor outcomes.  
 
So since there are many possible futures, your morning commute is a 5D rat race.  

# Warning:
I trust my code and it works appears to work as intended, but you should quality check it if injuries could result from malfunction.  Two obvious uses of 5d Lee's algorithm are continuously-refreshed autonomous vechicle/drone navigation and much the same for multiple robotic arms with overlapping reach playing "keep working but don't touch any other robots or humans."   Please extensively code review & QC before crushing the humans, ok!  

I used this algorith to drive a "scared ghost that avoids the kids and pets and tries to hide or blink out of existance" animated laser projection on my lawn for halloween from a window in my home.  Just fine year 1 outside.  Year 2 was cold/damp so I projected from an interior window... and reflections resulted in camera blind spots for flat scared laser ghost's behavior.  Even though the laser was not quite as bright as sunlight, I still didn't want it to touch traffic or kids.  So don't underestimate tiny sugar infused vampires that seek to trample a ghost, or expect care free real world use.  (unpublished hodge-podge of camera capture c, python for laser dmx512 control off a raspberry pi off usb port,  processing language for image processing & lee's algorithm & laser ghost squigle/emotion animation, mathematica to turn drawings into laser animation recorded in processing as heaps of numbers. End result: tiny vampire laser pointer cat toy.) 

# Why?  
I find certain bits of math and programming beautiful.  The juggle of pointers and branching paths in Lee's algorithm seemed like a
fun thing to try as a rust beginner- and my opinion at the time was if I couldn't write Lee's algorithm in Rust - I shouldn't program in Rust.  But Rust-lang won against my doubts!  And so I share.

# How to use:
I'm writing this readme a few years after coding Leemaze.  As I recall, (1) make a binary bit maze as a boolean vector.  (2) make a list of allowed moves. (3) Define a start & finish. (4) run leemaze function and get back a path of allowed moves for guidance.

The code has better documentation than this readme... but maybe not a few of these memories.

Enjoy - Dusty D.
