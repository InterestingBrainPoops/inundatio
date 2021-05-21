# inundiato
A Rust Battlesnake built using the warpy_snake template.

# How it works
It iterates through the 4 possible board states from the current one, and then sees if:
1. its dead
2. it won   
These 2 are the extremes.  
Then it looks for the following:  
1. Smallest path to the nearest food using A*
2. Smallest path to the closest snake shorter than it
3. Food ownership (Vornoi w/ A*)
4. Area ownership (Vornoi w/ A*)


# Current bugs
Dies from low health
Goes for obviously unreachable food