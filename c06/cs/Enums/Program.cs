Console.WriteLine($"IPV4 is version {IpAddrKind.V4.Version()}");
Console.WriteLine($"IPV4 is version {IpAddrKind.V6.Version()}");

Capabilities current = Capabilities.Floating | Capabilities.Transfer;
// This remarkably formats already.
Console.WriteLine($"Current capabilities are {current}");

// Enums can only be one of various numeric types. You cannot make enums have methods or any other kinds of behaviour.
public enum IpAddrKind {
	V4,
	V6
}

// Although extension methods give you the capability of pretending like enums have methods. They cannot encode any more value than what they are though.
public static class Extensions {
	// Bonus points, there's a warning for this switch expression if it doesn't handle all cases (exhaustive).
	public static int Version(this IpAddrKind ip) => ip switch {
		IpAddrKind.V4 => 4,
		IpAddrKind.V6 => 6,
		_ => 0
	};
}

// Enums do have flag behaviour for bitwise operations though which is nice.
[Flags]
public enum Capabilities {
	None = 0,
	Carry = 1,
	Transfer = 2,
	Floating = 4
}