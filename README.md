# FLOODFILL IN RUST

This is a simple floodfill algo in Rust. The game reads a maze from a file, navigates through it, and tries to find the target ('X') by exploring the available spaces.

## How to Play

1. Ensure you have Rust installed on your system.
2. Create a text file containing the maze layout. Use the following characters:
   - Space (' '): Represents an empty space.
   - 'O': Represents the starting position.
   - 'X': Represents the target.
   - Other characters: Represent walls or obstacles.
   
   Note: The maze should be a rectangular grid with consistent row and column lengths.
3. Save the maze file with a ".txt" extension.
4. Open a terminal or command prompt and navigate to the project directory.
5. Run the game using the following command:

   ```shell
   cargo run <path_to_maze_file.txt>
   ```
