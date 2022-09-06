var x = "Banana!"; // Type is implicit when var is used.
Console.WriteLine($"The value of x is: {x}");
PrintTypeOf(x);

// int x = 4; // Not allowed to redfine the variable.
int x2 = 4;
Console.WriteLine($"The value of x2 is: {x2}");
PrintTypeOf(x2);
x2 = 6; // Reassigning is always valid.
Console.WriteLine($"The value of x2 is: {x2}");

// Local variables can't be marked as readonly, so you can't have immutable local variables.
const int A_CONSTANT = 2 * 3 * 5;
Console.WriteLine($"The value of A_CONSTANT is: {A_CONSTANT}");

// There's no C# equivalent of isize or C++'s size_t. `int` is always 32-bits. You do get these types though.
Int32 x3 = 400;
Console.WriteLine($"The value of x3 is: {x3}");
PrintTypeOf(x3);

// Shadowing is illegal in scopes as well because it hides outer variables.
// {
// 	double x = 10.5;
// 	Console.WriteLine($"The value of x is: {x}");
// 	PrintTypeOf(x);
// }

int x4 = 0x12345678; // Overflows will just happen silently.
x4 *= 10;
Console.WriteLine($"The value of x4 is: {x4}");

// Emojis that are wider than 2 bytes can't fit in char. Which is most of them.
char c = '♠';
Console.WriteLine($"The value of c is: {c}");
PrintTypeOf(c);
string s = "🦎";
Console.WriteLine($"The value of s is: {s}");
PrintTypeOf(s);
Console.WriteLine($"The value of s[0] is: {s[0]}"); // It's valid but at what cost?
PrintTypeOf(s[0]);

var t = (1, 5.0, "Hi");
Console.WriteLine($"The value of t is: {t}"); // This actually prints!
PrintTypeOf(t);

var (a, b, d) = t;
Console.WriteLine($"The value of a is: {a}");
Console.WriteLine($"The value of b is: {b}");
Console.WriteLine($"The value of d is: {d}");

Console.WriteLine($"The value of t.Item1 is: {t.Item1}");
Console.WriteLine($"The value of t.Item2 is: {t.Item2}");
Console.WriteLine($"The value of t.Item3 is: {t.Item3}");

int[] arr = {5, 3, 1, 2, 4};
Console.WriteLine($"The value of arr is: {arr}"); // This is not a useful print.
PrintTypeOf(arr);
Console.WriteLine($"The value of arr[0] is: {arr[0]}"); // This is not a useful print.
// Console.WriteLine($"The value of arr[5] is: {arr[5]}"); // Array out of bounds at runtime.

// It's not really an expression but...
var f = () => {
	int x = 3;
	return x + 1;
}; // This tells you off if you try and immediately call it.
int r = f();
Console.WriteLine($"The value of r is: {r}");
PrintTypeOf(r);

Console.WriteLine($"The value of CoolFunction() is: {CoolFunction()}");
PrintTypeOf(CoolFunction);
Console.WriteLine($"The value of LameFunction() is: {LameFunction()}");
PrintTypeOf(LameFunction);

/*
  Multiline comments exist.
*/

if (a == 1) {
	Console.WriteLine("a == 1");
} else {
	Console.WriteLine("This shouldn't run!");
}

bool b2 = true;
if (b2) {
	Console.WriteLine("Bools work!");
}

int counter = 0;
// There's not really an equivalent for evaluating a loop.
while (true) { // Kinda the same as "loop"?
	++counter;
	Console.WriteLine($"The value of counter is: {counter}");

	if (counter >= 10) {
		break;
	}
}

while (counter < 15) {
	++counter;
	Console.WriteLine($"The value of counter is: {counter}");
}

// Iterating over an enumerable has a special keyword here.
foreach (int element in arr) {
	Console.WriteLine($"The value of element is: {element}");
}

// And no emoji names.
// int 🚍 = 5;


void PrintTypeOf<T>(T t) where T : notnull {
	Console.WriteLine($"Its type is: {t.GetType()}");
}

// These one liner functions don't exist in Rust.
int CoolFunction() => 600;

int LameFunction() {
	// 500; // C# tells you off for useless lines like this.
	return 700;
}