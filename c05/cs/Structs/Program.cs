// C# supports this complex structure initialising for any public fields. Works for classes and records too.
User user1 = new() {
	Email = "someone@example.com",
	Username = "someusername123",
	Active = true,
	SignInCount = 1
};
Console.WriteLine(user1); // Prints something not really useful. You need to override ToString.
Console.WriteLine(user1.ToStringExtension());

// C#'s structs are mutable and value based. Equals operators do a full copy into a new variable.
User user2 = user1;
user2.Username = "adifferentusername456";
user2.Active = false;

Console.WriteLine(user2.ToStringExtension());
Console.WriteLine(user1.ToStringExtension());

// C# also has records, immutable value based structures. They also already have Equals() implemented (whereas structs don't) (along with GetHashCode() and the == and != operators).
UserRecord user3 = BuildUser("fsinatra@email.com", "chairmanoftheboard");
UserRecord user4 = user3 with { Email = "epresley@email.com" }; // This with syntax is exclusive to records.

// Also this really cheesey syntax.
Person person = new("Jon", "Anderson");
var (firstName, lastName) = person;
Console.WriteLine($"Successfully decomposed {firstName} {lastName}");

Rectangle rect1 = new Rectangle {
	Width = 30,
	Height = 50
};
Console.WriteLine($"The area of the rectangle is {rect1.Area} square pixels.");

Rectangle rect2 = new Rectangle {
	Width = 20,
	Height = 40
};
Console.WriteLine($"Can rect1 hold rect2? {rect1.CanHold(rect2)}");

Rectangle rect3 = Rectangle.Square(40);
Console.WriteLine($"Can rect1 hold rect3? {rect1.CanHold(rect3)}");

UserRecord BuildUser(string email, string username) => new() {
	Email = email,
	Username = username,
	Active = true,
	SignInCount = 1
};

struct User {
	// This is actually a property and not a method. Properties are like fields but they implicitly are hiding a getter and setter method.
	public bool Active { get; set; }
	public string Username { get; set; }
	public string Email { get; set; }
	public ulong SignInCount { get; set; }
}

record UserRecord {
	public bool Active { get; set; }
	// C# warns that this nonnull class field needs an initialiser, and most classes "default" to null, so I've put empty strings on it. You can't exactly assert this on all usage unless you use a custom constructor.
	public string Username { get; set; } = string.Empty;
	public string Email { get; set; } = string.Empty;
	public ulong SignInCount { get; set; }
};

record Person(string FirstName, string LastName);

struct Rectangle {
	public int Width { get; set; }
	public int Height { get; set; }

	// This is also a property but the getter is defined as the following "method", and the setter is disabled (unless it refers exactly to one field).
	public int Area => Width * Height;

	public bool CanHold(Rectangle other) => Width > other.Width && Height > other.Height;

	public static Rectangle Square(int size) => new() {
		Width = size,
		Height = size
	};
}

// Thanks https://stackoverflow.com/a/9299333 for this shorthand.
// Also we have extension methods: they're not defined on the class, but can be called as if they were. It's shorthand so it's not doing anything too fancy.
public static class Extensions {
	public static string ToStringExtension(this object obj) {
		System.Text.StringBuilder sb = new();
		foreach (System.Reflection.PropertyInfo property in obj.GetType().GetProperties()) {
			sb.Append(property.Name);
			sb.Append(": ");
			if (property.GetIndexParameters().Length > 0) {
				sb.Append("Indexed Property cannot be used");
			}
			else {
				sb.Append(property.GetValue(obj, null));
			}
			sb.Append(System.Environment.NewLine);
		}
		return sb.ToString();
	}
}