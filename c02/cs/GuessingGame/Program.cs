// No imports for this apparently.

Console.WriteLine("Guess the number!");

// Need to construct a Random object.
Random rand = new();

// Can't make local variables immutable. Class members can have `readonly`, but local variables can't.
// Also a little different to the range based generator but it's functionally equivalent.
int secretNumber = (int)rand.NextInt64(100) + 1;

while (true) {
	Console.WriteLine("Please input your guess.");

	string? guessStr = null;
	try {
		guessStr = Console.ReadLine();
	} catch (IOException) {
		Console.Error.WriteLine("Failed to read line");
		// Manually exiting the environment.
		Environment.Exit(1);
	}

	// Console.ReadLine() returns `string?`, which means we've got a null case in the mix.
	if (guessStr is null) continue;

	// Fortunately C# knows `guessStr` is nonnull from here on.
	if (!int.TryParse(guessStr, out int guess)) continue;

	Console.WriteLine($"You guessed: {guess}");

	// I'd love to use `guess.CompareTo(secretNumber) switch {}` but you can't do multiple statements in one case.
	if (guess < secretNumber) Console.WriteLine("Too small!");
	else if (guess > secretNumber) Console.WriteLine("Too big!");
	else {
		Console.WriteLine("You win!");
		break;
	}
}